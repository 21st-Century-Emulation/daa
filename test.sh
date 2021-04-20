docker build -q -t daa .
docker run --rm --name daa -d -p 8080:8080 daa

sleep 5

RESULT=`curl -s --header "Content-Type: application/json" \
  --request POST \
  --data '{"opcode":187,"state":{"a":155,"b":1,"c":66,"d":5,"e":5,"h":10,"l":2,"flags":{"sign":false,"zero":false,"auxCarry":false,"parity":false,"carry":false},"programCounter":1,"stackPointer":2,"cycles":0}}' \
  http://localhost:8080/api/v1/execute`
EXPECTED='{"opcode":187,"state":{"a":1,"b":1,"c":66,"d":5,"e":5,"h":10,"l":2,"flags":{"sign":false,"zero":false,"auxCarry":true,"parity":false,"carry":true},"programCounter":1,"stackPointer":2,"cycles":4}}'

docker kill daa

DIFF=`diff <(jq -S . <<< "$RESULT") <(jq -S . <<< "$EXPECTED")`

if [ $? -eq 0 ]; then
    echo -e "\e[32mDAA Test Pass \e[0m"
    exit 0
else
    echo -e "\e[31mDAA Test Fail  \e[0m"
    echo "$RESULT"
    echo "$DIFF"
    exit -1
fi