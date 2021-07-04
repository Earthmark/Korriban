# Korriban Runtime

## The core runtime dependency chain

Field Reservation (type agnostic)
    Scheduling information. Contains field indexes.
    Indexes element reservations to field sets.
    Defines active interdependency set
    DOES NOT DEFINE FIELDS ON ELEMENTS (only between).
    Says what exit properties go where (and if they should (binding chain)).
    Asked during update loop.
    Used by scheduler for interdependecy data? Possibly is scheduler?
Prop Set (strong type) - Data storage, requires fast R/W.
    Very dumb data storage.
    Delta frame based.
Element - Reserves field indicies, R/W to storage quickly.
    Asks reservation for I/O
    Prepares binding sites
    More to insert here as layers come online.
    Invokes body

Element reserves by asking Field Reservation for index, then strong typing R/W to prop set trait.
    Wasm Element interface - Map from field reservation index to local index for each type (set is primary keyed by type to compiler switches).

Reference types only make sense in the context of another module.
    What makes a reference type?
