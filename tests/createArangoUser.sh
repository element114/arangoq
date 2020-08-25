curl -X POST --header 'accept: application/json' --user root:test_password --data-binary @- --dump - http://localhost:8529/_api/user <<EOF
{
	"user" : "test_dev",
	"passwd" : "test_dev_pw"
}
EOF