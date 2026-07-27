# vendor/textual/tree

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **0%** (chrome-excluded; 428 non-blank content cells)

## Before (color-stripped)

```
 ⭘                                 TreeApp                                      
▼ Root                                                                          
└── ▼ {} JSON                                                                 ▁▁
    ├── code='5060292302201'                                                    
    ├── ▼ {} product                                                            
    │   ├── _id='5060292302201'                                                 
    │   ├── ▶ [] _keywords                                                      
    │   ├── ▶ [] added_countries_tags                                           
    │   ├── ▶ [] additives_debug_tags                                           
    │   ├── additives_n=2                                                       
    │   ├── additives_old_n=2                                                   
    │   ├── ▶ [] additives_old_tags                                             
    │   ├── ▶ [] additives_original_tags                                        
    │   ├── ▶ [] additives_prev_original_tags                                   
    │   ├── ▶ [] additives_tags                                                 
    │   ├── additives_tags_n=None                                               
    │   ├── allergens='en:milk'                                                 
    │   ├── ▶ [] allergens_debug_tags                                           
    │   ├── allergens_from_ingredients='en:milk, milk'                          
    │   ├── allergens_from_user='(en) en:milk'                                  
    │   ├── ▶ [] allergens_hierarchy                                            
    │   ├── ▶ [] allergens_tags
```

## After — compact

```
application @0,0,80,23
  heading @0,0,80,22 ⭘                                 TreeApp ▼ Root └── ▼ {} JSON                                                                 ▁▁ ├── code='5060292302201' ├── ▼ {} product │   ├── _id='5060292302201' │   ├── ▶ [] _keywords │   ├── ▶ [] added_countries_tags │   ├── ▶ [] additives_debug_tags │   ├── additives_n=2 │   ├── additives_old_n=2 │   ├── ▶ [] additives_old_tags │   ├── ▶ [] additives_original_tags │   ├── ▶ [] additives_prev_original_tags │   ├── ▶ [] additives_tags │   ├── additives_tags_n=None │   ├── allergens='en:milk' │   ├── ▶ [] allergens_debug_tags │   ├── allergens_from_ingredients='en:milk, milk' │   ├── allergens_from_user='(en) en:milk' │   ├── ▶ [] allergens_hierarchy │   ├── ▶ [] allergens_tags
```

## After — toon

```
application
  heading: ⭘                                 TreeApp ▼ Root └── ▼ {} JSON                                                                 ▁▁ ├── code='5060292302201' ├── ▼ {} product │   ├── _id='5060292302201' │   ├── ▶ [] _keywords │   ├── ▶ [] added_countries_tags │   ├── ▶ [] additives_debug_tags │   ├── additives_n=2 │   ├── additives_old_n=2 │   ├── ▶ [] additives_old_tags │   ├── ▶ [] additives_original_tags │   ├── ▶ [] additives_prev_original_tags │   ├── ▶ [] additives_tags │   ├── additives_tags_n=None │   ├── allergens='en:milk' │   ├── ▶ [] allergens_debug_tags │   ├── allergens_from_ingredients='en:milk, milk' │   ├── allergens_from_user='(en) en:milk' │   ├── ▶ [] allergens_hierarchy │   ├── ▶ [] allergens_tags
```

<details><summary>After — json</summary>

```json
{
  "role": "application",
  "rect": [
    0,
    0,
    80,
    23
  ],
  "children": [
    {
      "role": "heading",
      "value": "⭘                                 TreeApp ▼ Root └── ▼ {} JSON                                                                 ▁▁ ├── code='5060292302201' ├── ▼ {} product │   ├── _id='5060292302201' │   ├── ▶ [] _keywords │   ├── ▶ [] added_countries_tags │   ├── ▶ [] additives_debug_tags │   ├── additives_n=2 │   ├── additives_old_n=2 │   ├── ▶ [] additives_old_tags │   ├── ▶ [] additives_original_tags │   ├── ▶ [] additives_prev_original_tags │   ├── ▶ [] additives_tags │   ├── additives_tags_n=None │   ├── allergens='en:milk' │   ├── ▶ [] allergens_debug_tags │   ├── allergens_from_ingredients='en:milk, milk' │   ├── allergens_from_user='(en) en:milk' │   ├── ▶ [] allergens_hierarchy │   ├── ▶ [] allergens_tags",
      "rect": [
        0,
        0,
        80,
        22
      ]
    }
  ]
}
```

</details>

