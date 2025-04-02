This Code is a custom module for string custom runtime values in Redis. This does not allow normal get commands to fetch these vales

SAMPLE EXECUTION 

```
127.0.0.1:6379> MODULE LOAD /home/anupkmr/workplace/redis-private-store/target/release/libredis_private_store.so
OK
127.0.0.1:6379> SECURE.SET ddb_Credentials SampleDDBCredentials
OK
127.0.0.1:6379> SECURE.GET ddb_Credentials
"SampleDDBCredentials"
127.0.0.1:6379> SECURE.SET Password Password
OK
127.0.0.1:6379> SECURE.KEYS
1) "ddb_Credentials"
2) "Password"
127.0.0.1:6379> SECURE.DEL Password
(integer) 1
127.0.0.1:6379> SECURE.GET Password
(nil)
127.0.0.1:6379> SECURE.KEYS
1) "ddb_Credentials"
127.0.0.1:6379> GET ddb_Credentials
(nil)
127.0.0.1:6379>
```
