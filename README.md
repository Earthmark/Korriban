# Korriban


Structure heiarchy

Application
    Templates - WASM byte code
        Required Extensions
        Name
        API Version
    Instance - Primary instance
        Name
        Type Mapping - Incrementing recursive list of type names.
        Instance - WASM Program Space
            Component - Item in instance
            Service Provider - API Extension

Service
    Props - Values to interconnect the world, the main API surface.
    Networking - Net calls
    Updateable - For managing execution styles.
    Prop Weaver - updater, synchronizes dependencies.
    Heiarchy - order the world into a tree
    Librarian - categorize and manage wasm and native libraries.
    Components - Manages the lifetime of components - might be de-duped by librarian and weaver
    Labelmaker - Owns the mapping of types to keys, as well as generalizing in inheritence of types.

ApplicationService
    SessionService
        LibraryService

Application
    Session
        Instance

# Nouns and what they mean

## Universe
Global state, this is created as part of application startup, and it runs until application close. The termination of a universe results in the termination of the program. Universe level services are not required to be able to restart (although they should for testability).

Application state may include global credential management, cache management, compiler configuration, interfaces into external systems like the filesystem and web, and compiled wasm libraries that define elements.

## Galaxy
A unit that represents a lifetime of 'active' data, this is the first scope that has a representation of 'time' in the form of an update loop. A universe is about the rules that can govern a world, but the world itself is an instance backed by those rules.

World state may include the fields that represent the world, world specific credentials, Instances of wasm memory representing the elements of a world, and the execution graph of how elements are sequenced in a world.

## Elements
The representation of a primitive operation that makes up a world, an element is a function that consumes and produces property values.

`this is being rethought, as the distinction may not be useful, see 'root in Element Sequencing`

There are a few core kinds of elements:
 * A root element represents a consumer of fields, and causes upstream elements to be executed.
 * A transient element represents a function that consumes and produces fields, but has no side effects.

An element may elect to have different triggering criteria, such as only activating on a subset of input field changes, or being forced as active even if not dependent of a root element.

The sequencing and execution of elements is the main purpose of this project.

Root elements can represent interfaces into other systems, for instance rendering elements could consume the imputs to be displayed to the screen, or physics elements could consume transforms to construct physics simulations.

Systems may include both root and transient elements, for instance in physics root elements may construct the physical space, but a transient element may query that space with a raycast. The difference being the root element defines part of the external construct, but the transient element reads from that construct.

In general these rules follow:
 * When active, a root element is always executed after any dependent elements.
 * A transient element is executed at least when one of it's outputs is the head of a drive chain.
 * A root element may not have external side effects, or those effects may be internal to the element.
 
For codified rules, see Element Sequencing.
A complex case, the Neos smooth lerp node: This component represents what may be a constantly changing value, and may need to continously recompute based on inputs even if it is not currently a drive chain head. However an element representing that component could only need to watch the target value, due to if it is at that value then the speed no longer causes cascading updates. Such an element may also 

## Fields
A singular value in a World. This is what is persisted on save/load, and what is depended between. Fields can be literal values like an i32 or Mat3, or they can be reference types. In the physical analogy, a field is space where elements can interact with eachother.

Fields can be driven by a drive chain, a driven field means it is a destination of an Element. Driven fields can be driven by multiple elements, but only the front of the drive chain is actually used.

Field can be deffered, meaning it has a drive chain and a value, but the value is from the previous update loop.

# Element Sequencing
One of the main goals of this project is efficient element sequencing, and resequencing based on changes to the element graph.

A graph is constructed of elements, where the fields interconnecting elements represent edges, forming a directed acyclic graph of elements in the form of operators.

In this graph root nodes are represented as forard and backward, describing the flow of data. Root nodes are as far forward as possible. Spatially, nodes are executed from back to front, where the farthest forward node is executed last.

Currently the sequencing operation has two phases, back checking and forward execution.

## Approaches

Each of these are different attempts at solving the elements sequencing problem.
Each of them are given a DAG `M`, and a set of root nodes `n` in `M` that must be executed.
Every root node must be executed with all of it's dependencies satisfied.

Givens
 * The previous value of an execution is provided to compare against (it is possible to tell if something changed).

The maximations are as follows:
 * Use as little CPU as possible.
 * Handle adding elements efficiently
 * Handle removing elements efficiently.
 * Handle duplicating a set of elements efficiently.
 * Execute nodes in as clustered a way as possible.

Restrictions are as follows:
 * In general, elements must be executed in their totality (no re-entrant wasm).

### Back propagation
    while take node in n
        Evaluate n

    Evaluate : n -> value
        For every watched property wp in n
            Evaluate wp
        if a property changed
        for every non watched property nwp in n
            Evaluate nwp
        Execute local logic with wp and nwp
        return executed logic result
    
Pros:
 * Simple to execute
 * Runs on the core concept of the DAG, relying on the graph itself.

Cons:
 * Likely needs to execute to all leaf nodes to ensure no stale values, although that check can be done linearly around components.

## Root Down Sequencing

## Leaf Up Sequencing

Given a graph of nodes, and a set of root nodes. 

## Notes

Root elements are required to always execute if active, but transient elements are required to update only when they would modify an input field of a root element.

A root element may opt to be a transient element, such as with an active flag, but that is done by the root element only depending on the active flag's value, watching for when it goes back to true, at which point the root element goes back to depending on the other input fields.

The following notation is to be used when defining an element.

`E : (ident(+?))* -> (ident(<f>?))`

Inputs are makred as observed when it has a + at the end of the input identifier. Some globally defined outputs exist, such as `root`

A root element is defined as `'root : (inputs*) -> (root, Output*)`. `root` is a psuedo element that always is counted as requiring re-execution.

### The smooth lerp problem

Elements may choose to change what fields are being observed at any given time. For instance
`SmoothLerp : (target+, speed+) -> (value)` in a general case, however when target = value, the element is redefined as `SmoothLerp : (target+, speed) -> (value)` where speed is no longer observed.

This isn't valid, previously this was `t` based but that pivoted to a root output. This may require dynamically changing outputs, but that requires a reconstruction of the dependency graph.

On second thought, `t'` is a good approach, however it may execute mode more nodes than expected.

Two pass possibly, one for `+` properties, and one for any activated properties past that.
`+` fields are forced to always execute, because they may be able to be used at any point, and that's the point of a `+` operator. This resolves the many execution problem, because it's not a many execution problem. Also, having an active on the smooth lerp allows bypassing activation anyways.

Smooth lerp gets around this restriction, due to me having the siguature of `SmoothLerp : (active+, target, speed, t) -> (value)`. From this, `t` must be provided as a field, which makes more sense for components being pure virtual.

I don't like this smooth lerp signature, t implies it's going to update quite a lot. Nevermind, it won't update extra ticks. It needs time as a passive component to know that it needs to update in the first place. Smooth lerp does need to update on tick, as its value may be dependend upon downstream.

Core issue is that some fields do not matter in the execution graph, in particular `speed` no longer matters once `value = target`. Propose adding `-`, to denote that a field is depended upon, but a change in value does not trigger an invocation.

New signature chain is `SmoothLerp : (active+ target, speed) -> (value)` in a general case, however when target = value, the element is redefined as `SmoothLerp : (active+, target, speed-) -> (value)`.

`-` fields are likely a mistake, and should not be done. Any point where - needs to be removed, would require a deferment. This is not overly useful, and breaks the "when a node is to run, all dependencies are ready" rule. This may be reinstated later.

Roots need a seconday update tick? Roots do not, `+` observing a signal source means all frequency based clocks are possible implicitly. `+` means a node is rooted by the activation of the root dependency.

### Psudo fields

To update any system interaction, a psudo field needs to be added as a virtual field.

Using this a root element may be toggleable using this semantic: `'root : (active+, inputs*) -> (Output*)` when active is true, but it changes to `'root : (active+, inputs*) -> (Output*)` when active is false. Active is still observed, but is not updated due to t. The element may still be executed due to dependent outputs however.


### Breaker Elements
`Breaker : (active+, token+?) -> root`
A breaker takes an active signal, and if true causes the token to cause dependencies to flow.

Breaker elements are no longer needed due to having both `-` and `+` field kinds.

As `-` fields don't exist now, the breaker concept may be needed again.


### Defer Element
Defer can be handled with virtual fields, and encoding a continuation id in a `+` field.

### Element Sequencing Logic
AKA: If, while, and so on. This may also resolve part of what `-` was trying to solve.

If can be represented by an incrementing field between two elements. This requires the path be included in the execution graph, which means the tail is a root. This is problematic as it will enforce that every side property along the graph is executed.

Possibly a new rule is required.

For each `+` field, if an upstream `+` field exists, that acts as a circuit breaker...

this does not work with smooth lerp as it would not have a changed `+` field and could be used as an input to a `+` element.

`++` fields could be added, an "If this field did not change, do not execute this element".
This adds execution tiers.

The pattern to do something on an action is quite powerful, and should be done responsibly.
There is also the issue of sequencing.


