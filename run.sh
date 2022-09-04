create_dir="mkdir ./dist"
eval $create_dir

extract="tar -xvf dist.tar -C ./dist"
eval $extract

run_server="docker run --name test-nginx -v \$(pwd)/dist:/usr/share/nginx/html -p 8080:80 -d nginx"
