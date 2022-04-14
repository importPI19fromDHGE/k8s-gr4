#!/bin/bash
echo -e "[GET /]:\n\t$(curl -s $1)"
echo -e "\n[POST /]:\n\t$(curl --header "Content-Type: application/json" \
     --request "POST" \
     --silent \
     --data {\"content\":\"test-todo\"} \
     $1)"
echo -e "\n[GET /{id}]:\n\t$(curl -s $1/$2)"
echo -e "\n[DELETE /{id}]: \n\t$(curl -s --request "DELETE" $1/$2)"
