 param (
    [int]$numberOfClients = 1
 )

invoke-expression 'cmd /c start powershell -Command { cd ./file_server/; cargo run }';


For ($i=0; $i -le $numberOfClients; $i++) {
    invoke-expression 'cmd /c start powershell -Command { cd ./file_client/; cargo run }';
}

