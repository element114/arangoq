curl -X POST --header 'accept: application/json' --user root:test_password --data-binary @- --dump - http://localhost:8529/_api/database <<EOF
{
  "name" : "test_db",
  "users" : [{
    "username" : "test_dev",
    "passwd" : "test_dev_pw"
  }],
  "options" : {
    "sharding" : "flexible",
    "replicationFactor" : 3
  }
}
EOF