import os
from functools import wraps
from inspect import isawaitable

import nanoid
from itsdangerous import BadSignature, TimedSerializer
from sanic.exceptions import SanicException
from sanic.exceptions import Unauthorized as SanicUnauthorized

from oauthpy import defaults

SECRET = os.environ["SECRET"]
max_age_seconds = 120


class WebAuthFailed(SanicException):
    status_code = 401
    quiet = True

    def __init__(self, message="Authentication failed.", **kwargs):
        super().__init__(message, **kwargs)


def sessionid_factory(salt="cookies"):
    s = TimedSerializer(SECRET, salt=salt)
    session = s.dumps(nanoid.generate(size=6))
    return session


def validate_session(cookie, max_age_seconds=30, salt="cookies"):
    s = TimedSerializer(SECRET, salt=salt)
    return s.loads(cookie, max_age=max_age_seconds)


def cookie_protected(max_age=max_age_seconds):
    """verify a token from a request.
    Optionally if a list of scopes is given then will check that scopes
    with the scopes provided by the token.
    :param scopes: a list of scopes
    :param required_all: if true it will check that the all the names provided
    match with the required.
    """

    def decorator(f):
        @wraps(f)
        async def decorated_function(request, *args, **kwargs):
            cookie = request.cookies.get(defaults.COOKIE_SESSION_KEY)
            if not cookie:
                raise WebAuthFailed()
            try:
                sess = validate_session(cookie,
                                        max_age_seconds=max_age,
                                        salt="cookies")
                request.ctx.sessionid = sess
                response = f(request, *args, **kwargs)
                if isawaitable(response):
                    response = await response

            except BadSignature as e:
                raise WebAuthFailed() from e
            return response

        return decorated_function

    return decorator
