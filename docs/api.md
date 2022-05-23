# API Documentation
## Introduction
The base URL for every api request should be `https://<host-ip>/`, where the `<host-ip>` is the IP of the machine hosting the Quantum install, and any arguments should be passed in an `application/json` request, responses will also be in such format.

## Controls
### `POST` `/api/controls/next_slide`
Advanced to the next slide

### `POST` `/api/controls/prev_slide`
Go to the previous slide

### `POST` `/api/controls/present_item`
#### Arguments
argument|description|type
-|-|-
`item_no`|Which item of the schedule to present. Starts from `0`.|`int`

#### Response
key|description
-|-
`title`|Title of selected item