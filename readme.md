## Rick and Morty CLI

## Features
- Implemented a custom query language for the Rick and Morty API
- Query all entties - characters, locations, episodes
- Manipulate query results with filters and sorting operations
- Multi-threaded; Seperate render thread from main thread, allowing for a more responsive UI (non-blocking)


## Query Language
There are two kinds of operations: operations for constructing a query, and operations for manipulating the query results after the query has been executed.

### Query Operations
- Query all entities
    - `CHARACTERS` 
    - `LOCATIONS`
    - `EPISODES`
- NAME(string)
    - `CHARACTERS::NAME(rick)`
- PAGE(number)
    - `CHARACTERS::PAGE(1)`
    - `LOCATIONS::PAGE(1)`
- DIMENSION(string)
    - `LOCATIONS::DIMENSION(C-137)`

### Manipulation Operations
- CONTAINS(string, string|number) - 
    - `CHARACTERS::CONTAINS(name, rick)`
    - `CHARACTERS::CONTAINS(status, ive)`
    - `CHARACTERS::CONTAINS(species, man)`
    - `LOCATION::CONTAINS(name, C12)::CONTAINS(status, ive)::CONTAINS(id, 1)`
- SORT(ASC|DSC, string) -
    - `CHARACTERS::SORT(ASC, name)`
    - `CHARACTERS::SORT(DSC, id)`
    - `LOCATION::SORT(ASC, residents)` - Sorts by the number of residents in the location
    - `LOCATION::SORT(ASC, created)` - Sorts by the date the location was created 
- LIMIT(number) -
    - `CHARACTERS::LIMIT(10)`
    - `LOCATION::LIMIT(5)`
    - `EPISODE::LIMIT(1)`

All operations are chainable, and can be combined to create complex queries:
- `CHARACTERS::NAME(rick)::PAGE(1)::SORT(ASC, name)::LIMIT(10)::CONTAINS(name, pickle)`


