### Database Table Creation
```
CREATE TABLE esm_items (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    model TEXT UNIQUE NOT NULL 
);

```