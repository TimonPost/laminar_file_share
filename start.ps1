invoke-expression 'cmd /c start powershell -Command { cd ./file_server/; cargo run }';
invoke-expression 'cmd /c start powershell -Command { cd ./file_client/; cargo run }';