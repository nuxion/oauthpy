import os
from dataclasses import dataclass

from httpx_oauth.clients.google import GoogleOAuth2
from sanic import Sanic
from sanic.response import json, redirect, text

from oauthpy import defaults
from oauthpy.utils import cookie_protected, sessionid_factory

REDIRECT_URL = "http://localhost:8000/auth/google"
app = Sanic("MyHelloWorldApp")
# app.extend(config={
#    "templating_path_to_templates": f"{os.getcwd()}/oauthpy/templates"
# }
# )


google = GoogleOAuth2(
    os.environ["GOOGLE_CLIENTID"], os.environ["GOOGLE_SECRET"])
token = None


@app.get("/")
@app.ext.template("index.html")
async def hello_world(request):
    u = request.args.get("user")
    ut = request.cookies.get("user-test")
    print(ut)

    authenticated = False
    if ut:
        authenticated = True
        u = ut

    return ({"data": {"authenticated": authenticated, "user": u}})


@app.get("/logout")
async def logout_route(request):
    rsp = redirect("/")
    del rsp.cookies["user-test"]
    del rsp.cookies[defaults.COOKIE_SESSION_KEY]
    return rsp


@app.get("/secure")
@cookie_protected(max_age=10)
async def secure_route(request):
    return json("it's a secure route")


@app.get("/login/google")
async def loging_google_handler(request):
    url = await google.get_authorization_url(os.environ["AUTH_CALLBACK"])
    return redirect(url)


@app.get("/auth/google")
async def auth_google_handler(request):
    code = request.args.get("code")
    token = await google.get_access_token(code=code, redirect_uri=REDIRECT_URL)
    print(token)
    user = await google.get_id_email(token["access_token"])

    rsp = redirect("/")
    rsp.cookies["user-test"] = user[1]
    rsp.cookies["user-test"]["max-age"] = 120
    rsp.cookies[defaults.COOKIE_SESSION_KEY] = sessionid_factory()
    rsp.cookies[defaults.COOKIE_SESSION_KEY]["httponly"] = True
    rsp.cookies[defaults.COOKIE_SESSION_KEY]["max-age"] = 120

    return rsp


if __name__ == "__main__":

    app.run(
        auto_reload=False,
        debug=True,
        single_process=True
    )
