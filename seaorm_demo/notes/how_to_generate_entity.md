
```sh
sea-orm-cli generate entity \
--tables workload "key_feature" storage_type milestone workload_keyfeature storage_type_keyfeature milestone_keyfeature \  
-o entity/src/acstor \
--lib --with-serde both \
--model-extra-derives async_graphql::SimpleObject
```