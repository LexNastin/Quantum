import flask

class API:
    def __init__(self, shard):
        self.shard = shard
        self.api = flask.Blueprint('api', __name__, template_folder='templates')

        @self.api.route('/api/ping')
        def ping():
            return flask.request.method + " Pong"