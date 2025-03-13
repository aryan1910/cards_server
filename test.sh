#!/bin/bash

BASE_URL="http://localhost:8080"

echo "🟢 Testing: Get All Translations"
curl -s -X GET "$BASE_URL/translations" | jq
echo -e "\n-------------------------------------\n"

echo "🟢 Testing: Get a Random Translation"
curl -s -X GET "$BASE_URL/translation/random" | jq
echo -e "\n-------------------------------------\n"

echo "🟢 Testing: Get Translation by ID (id=1)"
curl -s -X GET "$BASE_URL/translation?id=1" | jq
echo -e "\n-------------------------------------\n"

echo "🟢 Testing: Get Translation by Invalid ID (id=100)"
curl -s -X GET "$BASE_URL/translation?id=100"
echo -e "\n-------------------------------------\n"

