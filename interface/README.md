# Korriban Interface
The C api for wasm modules to bind to.

## Some general guidelines:
1. Be self contained to one flavor of interface. (no random things in a bucket).
2. Have the module provide memory for the host to write into (no blind writes by the host).
3. Be rejectable, where when rejected the api acts in a sane way (user can turn off access to an api).
4. Do as much as possible with an allowed permission set (no wasted permissions, aka don't require web and only do addition).
5. Efficiency is key, test to ensure a compiler can optimize the call.
6. Ensure a module is re-entrant (no global memory).
7. Assume the module is compromized on the module side (Do not store user sensitive information on the module side, expose as little as possible as allowed by the permission set).
8. Whenever possible, be stateless.
9. Be predictable.

## Current interfaces

### Fields
* Reading / writing a single stack based value.
* 0 based Index, undefined as to what the base is relative to (currently it is per type, with seperate input and output channels).
* Pass a module address to write to.
