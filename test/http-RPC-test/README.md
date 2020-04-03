# JSON RPC Communication with qaul-http-RPC Interface

The scripts in this folder can be used to 

## Usage

### First: Create a User

Create a new user with password '1234456'

```bash
./users_create.sh
```

You'll get a JSON response that looks similar to this

```json
{"id":"1","method":"create","kind":"users","data":{"auth":{"id":"C735 AA5D 9FB6 8564 BE54 3508 5E0C 29FF DBA4 8618 4DFF 3525 AC54 DF45 F22A 75BB","token":"p6vr6KiVgPP4l5ZRgRLK2eWHKK7eQUHSwUC1PKVIGi7WSijTFa-bvn31ukGYHeQSBC7YOTuCYkjLqHbCyf7SZUqKdacpDVLCSc9TZY-rqyXXFeOaaB1xFtVIaVrZHS1IVHqEz3H94YDif3z7Z7JNdgMGPS_iU-AfIQ33FdPAgKfhFpLThjh98tWub4GME958fCeNDPwk-2PsMcPQEOsV4jc7N12PNE91OGdAEmj5v5XF1NLHwA-voYMuK4aM8-WXHkOLqG1ucCTuojoGJ1vrc4K5dNSesSSt8qMZH1qKPN__DDNWfdGg_-pjltIUJ_FhYoX-9lX9yntrJgTPAQI5FQ=="}}}
```

The data in the "auth" object the user `id` and the access `token` we'll need for each query of the interface. 

To have these two things available, we need set them as environment variables for the shell scripts. Set your "id" as `QAUL_ID` and the "token" as `QAUL_TOKEN`. Set them in your terminal with the following commands:

```bash
export QAUL_ID="C735 AA5D 9FB6 8564 BE54 3508 5E0C 29FF DBA4 8618 4DFF 3525 AC54 DF45 F22A 75BB"

export QAUL_TOKEN="p6vr6KiVgPP4l5ZRgRLK2eWHKK7eQUHSwUC1PKVIGi7WSijTFa-bvn31ukGYHeQSBC7YOTuCYkjLqHbCyf7SZUqKdacpDVLCSc9TZY-rqyXXFeOaaB1xFtVIaVrZHS1IVHqEz3H94YDif3z7Z7JNdgMGPS_iU-AfIQ33FdPAgKfhFpLThjh98tWub4GME958fCeNDPwk-2PsMcPQEOsV4jc7N12PNE91OGdAEmj5v5XF1NLHwA-voYMuK4aM8-WXHkOLqG1ucCTuojoGJ1vrc4K5dNSesSSt8qMZH1qKPN__DDNWfdGg_-pjltIUJ_FhYoX-9lX9yntrJgTPAQI5FQ=="
```

### Use the Authenticated Session

**For this scripts to work, you need to have set `QAUL_ID` and `QAUL_TOKEN` as described above.**

#### Set User Name

TODO: Crashes

```bash
./users_modify.sh
```

#### Get a List of all Users

```bash
./users_list.sh
```

### Create a Chat-Room

```bash
./chat-rooms_create.sh
```


#### Send a Message

