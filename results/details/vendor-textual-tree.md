# vendor/textual/tree

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **0%** (chrome-excluded; 428 non-blank content cells)

## Sizes

| encoding | bytes | tokens | vs plain (bytes) | vs plain (tokens) |
|---|---:|---:|---:|---:|
| raw | 5748 | 2954 | 0.39x | 0.16x |
| plain | 2222 | 468 | 1.00x | 1.00x |
| compact | 1053 | 384 | 2.11x | 1.22x |
| nogeo | 1031 | 367 | 2.16x | 1.28x |
| toon | 1032 | 368 | 2.15x | 1.27x |
| json | 1227 | 456 | 1.81x | 1.03x |

<details><summary>Before — raw ANSI (5748 bytes, escapes shown literally)</summary>

```
\x1b[0;48;2;36;47;56m \x1b[0;38;2;224;224;224;48;2;36;47;56m⭘\x1b[0;48;2;36;47;56m                                 \x1b[0;38;2;224;224;224;48;2;36;47;56mTreeApp\x1b[0;48;2;36;47;56m                                      \x1b[0m
\x1b[0;38;2;224;224;224;48;2;39;39;39m▼ Root\x1b[0;48;2;39;39;39m                                                                        \x1b[0;48;2;0;48;84m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m└── \x1b[0;38;2;224;224;224;48;2;39;39;39m▼ {} JSON\x1b[0;48;2;39;39;39m                                                                 \x1b[0;38;2;0;0;0;48;2;0;48;84m▁▁\x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    ├── \x1b[0;1;38;2;224;224;224;48;2;39;39;39mcode\x1b[0;38;2;224;224;224;48;2;39;39;39m=\x1b[0;38;2;152;224;36;48;2;39;39;39m'5060292302201'\x1b[0;48;2;39;39;39m                                                  \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    ├── \x1b[0;38;2;224;224;224;48;2;39;39;39m▼ \x1b[0;1;38;2;221;237;249;48;2;1;120;212m{} product\x1b[0;48;2;39;39;39m                                                          \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;1;38;2;224;224;224;48;2;39;39;39m_id\x1b[0;38;2;224;224;224;48;2;39;39;39m=\x1b[0;38;2;152;224;36;48;2;39;39;39m'5060292302201'\x1b[0;48;2;39;39;39m                                               \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;38;2;224;224;224;48;2;39;39;39m▶ [] _keywords\x1b[0;48;2;39;39;39m                                                    \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;38;2;224;224;224;48;2;39;39;39m▶ [] added_countries_tags\x1b[0;48;2;39;39;39m                                         \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;38;2;224;224;224;48;2;39;39;39m▶ [] additives_debug_tags\x1b[0;48;2;39;39;39m                                         \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;1;38;2;224;224;224;48;2;39;39;39madditives_n\x1b[0;38;2;224;224;224;48;2;39;39;39m=\x1b[0;1;38;2;88;209;235;48;2;39;39;39m2\x1b[0;48;2;39;39;39m                                                     \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;1;38;2;224;224;224;48;2;39;39;39madditives_old_n\x1b[0;38;2;224;224;224;48;2;39;39;39m=\x1b[0;1;38;2;88;209;235;48;2;39;39;39m2\x1b[0;48;2;39;39;39m                                                 \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;38;2;224;224;224;48;2;39;39;39m▶ [] additives_old_tags\x1b[0;48;2;39;39;39m                                           \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;38;2;224;224;224;48;2;39;39;39m▶ [] additives_original_tags\x1b[0;48;2;39;39;39m                                      \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;38;2;224;224;224;48;2;39;39;39m▶ [] additives_prev_original_tags\x1b[0;48;2;39;39;39m                                 \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;38;2;224;224;224;48;2;39;39;39m▶ [] additives_tags\x1b[0;48;2;39;39;39m                                               \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;1;38;2;224;224;224;48;2;39;39;39madditives_tags_n\x1b[0;38;2;224;224;224;48;2;39;39;39m=\x1b[0;3;38;2;244;0;95;48;2;39;39;39mNone\x1b[0;48;2;39;39;39m                                             \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;1;38;2;224;224;224;48;2;39;39;39mallergens\x1b[0;38;2;224;224;224;48;2;39;39;39m=\x1b[0;38;2;152;224;36;48;2;39;39;39m'en:milk'\x1b[0;48;2;39;39;39m                                               \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;38;2;224;224;224;48;2;39;39;39m▶ [] allergens_debug_tags\x1b[0;48;2;39;39;39m                                         \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;1;38;2;224;224;224;48;2;39;39;39mallergens_from_ingredients\x1b[0;38;2;224;224;224;48;2;39;39;39m=\x1b[0;38;2;152;224;36;48;2;39;39;39m'en:milk, milk'\x1b[0;48;2;39;39;39m                        \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;1;38;2;224;224;224;48;2;39;39;39mallergens_from_user\x1b[0;38;2;224;224;224;48;2;39;39;39m=\x1b[0;38;2;152;224;36;48;2;39;39;39m'(en) en:milk'\x1b[0;48;2;39;39;39m                                \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;38;2;224;224;224;48;2;39;39;39m▶ [] allergens_hierarchy\x1b[0;48;2;39;39;39m                                          \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;38;2;79;79;79;48;2;39;39;39m    │   \x1b[0;38;2;1;120;212;48;2;39;39;39m├── \x1b[0;38;2;224;224;224;48;2;39;39;39m▶ [] allergens_tags\x1b[0;48;2;39;39;39m                                               \x1b[0;48;2;0;0;0m  \x1b[0m
\x1b[0;48;2;0;48;84m            \x1b[0;48;2;0;0;0m                                                                    \x1b[0m
```

</details>

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

