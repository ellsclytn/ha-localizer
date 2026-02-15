# ha-localizer

A set of applications for providing timezone and geolocation information to Linux systems via Home Assistant & your smartphone.

---

## Applications

### ha-geoip

Provides geolocation information by relaying your smartphone's coordinates to Geoclue.

### ha-timezone-sync

Sets your system timezone (`/etc/localtime`) to the timezone of your smartphone.

## Setup

1. Both applications share configuration defined at `/etc/ha-localizer.toml`. To start, you'll need to [generate a long-lived access token in Home Assistant](https://developers.home-assistant.io/docs/auth_api/#long-lived-access-token). This value will be used as the `api_key` value in the configuration TOML.
2. A `device_id` is needed. This is the Home Assistant ID of the smartphone which will be used to relay information.
3. Ensure appropriate permissions are configured in the Home Assistant companion app. You will need 
  1. [Location sensors](https://companion.home-assistant.io/docs/core/location/) enabled if you want to relay your location to your Linux machine. You can verify this is working by going to your Home Assistant developer tools and checking if `device_tracker.<device_id>` is populated with coordinates.
  2. [Time zone sensor](https://companion.home-assistant.io/docs/core/sensors#current-time-zone-sensor) enabled if you want to use the timezone synchronization application. You can verify this is working by going to your Home Assistant developer tools and checking if `sensor.<device_id>_current_time_zone` is providing a `time_zone_id` attribute.

### Example config

```toml
# /etc/ha-localizer.toml
api_key="abc123"
device_id="foobar"
base_url="https://homeassistant.example.com" # no trailing slash
port=58573 # can be any free port. Only used by ha-geoip
```

## Usage

You can use one or both of the applications provided at your discretion.

### ha-geoip

Configure Geoclue to use the `ip` source in `/etc/geoclue/geoclue.conf`. Disable all other location sources in the file (unless you know what you're doing).

```ini
[ip]
enable=true
method=ichnaea
url=http://127.0.0.1:58573/
```

You will then need ha-geoip running. It is recommended to boot this on login. For example, a systemd user service may be used:

```ini
# ~/.config/systemd/user/ha-geoip.service

[Unit]
Description=ha-geoip

[Service]
ExecStart=/path/to/ha-geoip

[Install]
WantedBy=default.target
```

Then autostart the service with `systemctl --user enable --now ha-geoip`.

At this point, you should be able to see your location when running `/usr/lib/geoclue-2.0/demos/where-am-i`. If it merely hangs, you may need to set up a geoclue agent service as well:

```ini
# ~/.config/systemd/user/geoclue-agent.service

[Unit]
Description=Geoclue agent

[Service]
ExecStart=/usr/lib/geoclue-2.0/demos/agent

[Install]
WantedBy=default.target
```

Enable it with `systemctl --user enable --now geoclue-agent`.

### ha-timezone-sync

Timezone synchronization is only expected to be needed very occasionally (e.g. on boot, wake, or network change). For this reason, `ha-timezone-sync` exits as soon as a synchronization attempt finishes. Connecting to a network covers more than enough scenarios for when a timezone may change: it covers system boot, wake, connecting to a WiFi network in a new location, etc. For this reason, it's recommended to use a task scheduler which can respond to network change events.

**Example**

If you are using `systemd-networkd`, this can be achieved with the [`networkd-dispatcher`](https://gitlab.com/craftyguy/networkd-dispatcher) application.

```ini
# /etc/systemd/system/ha-timezone-sync.service

[Unit]
Description=ha-timezone-sync

[Service]
ExecStart=/path/to/ha-timezone-sync
Type=oneshot

[Install]
WantedBy=multi-user.target
```

```bash
# /etc/networkd-dispatcher/routable.d/10-sync-timezone

#!/usr/bin/env bash
systemctl start ha-timezone-sync
```
