import requests

html = requests.get("https://mediasaver.link", headers={"User-Agent":"Googlebot"}).text
open("z.html","w",encoding="utf-8").write(html)
