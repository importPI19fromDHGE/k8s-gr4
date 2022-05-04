#!/bin/bash
echo -e "[GET /]:\n\t$(curl -s --header "Authorized: $3" $1)"
echo -e "\n[POST /]:\n\t$(curl --header "Content-Type: application/json" \
     --header "Authorized: $3" \
     --request "POST" \
     --silent \
     --data {\"content\":\"test-todo\"} \
     $1)"
echo -e "\n[GET /{id}]:\n\t$(curl -s --header "Authorized: $3" $1/$2)"
echo -e "\n[DELETE /{id}]: \n\t$(curl -s --header "Authorized: $3" --request "DELETE" $1/$2)"
