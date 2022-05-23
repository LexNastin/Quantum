import flask

# :: throw error on wrong data type

class API:
    def __init__(self, shard):
        self.shard = shard
        self.api = flask.Blueprint('api', __name__, template_folder='templates')

        @self.api.route('/api/ping')
        def ping():
            return flask.request.method + " Pong"

        @self.api.route('/api/controls/next_slide', methods=['POST'])
        def next_slide():
            print(('next_slide',))
            self.shard.enqueue(('next_slide',))
            return '{}'

        @self.api.route('/api/controls/prev_slide', methods=['POST'])
        def prev_slide():
            print(('prev_slide',))
            self.shard.enqueue(('prev_slide',))
            return '{}'

        @self.api.route('/api/controls/present_item', methods=['POST'])
        def present_item():
            json = flask.request.json
            item = json.item_no
            print(('present_item',item))
            self.shard.enqueue(('present_item',item))
            # :: Return title
            return '{"title": "Not Yet"}'