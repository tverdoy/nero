import requests

for i in range(100_000):
    text = requests.get("http://127.0.0.1:8080/home").text
    if i % 1000 == 0:
        print(len(text))


