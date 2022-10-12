import os
from sanic import Sanic
from sanic.response import text
from httpx_oauth.clients.google import GoogleOAuth2

REDIRECT_URL = "http://localhost:8000/auth/google"
app = Sanic("MyHelloWorldApp")
google =  GoogleOAuth2(os.environ["GOOGLE_CLIENTID"], os.environ["GOOGLE_SECRET"])
token = None

@app.get("/")
async def hello_world(request):
    breakpoint()
    u = request.args.get("user")
    return text("Hello, world.")


@app.get("/auth/google")
async def auth_google_handler(request):
    code = request.args.get("code")
    token = await google.get_access_token(code=code, redirect_uri=REDIRECT_URL)
    print(token)
    user = await google.get_id_email(token["access_token"])
    print(user)

    return text("Hello, world.")


if __name__ == "__main__":

    app.run(
        auto_reload=False,
        debug=True
    )
