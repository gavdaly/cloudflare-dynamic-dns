# Cloudflare Dynamic DNS

I'm creating this so it's easy to use in a container, as a cron job, fass or manually.

Currently the only way to set the variables is with environment vars. The list is below.

## Usage

```sh
AUTH_EMAIL=%EMAIL% /
AUTH_KEY=%KEY% /
RECORD_ID=%RECORD_ID% /
ZONE_IDENTIFIER=%ZONE_IDENTIFIER % /
RECORD_NAME=%RECORD_NAME% /
cfddns
```

## To Do

- [ ] add cli with help
- [ ] implement logging
- [ ] get config from toml file with default location as `~/.config/cfddns/config.toml` or with a flag

### Future Usage

```sh
cfddns
```

when specifying the toml location

```sh
cfddns -c config.toml
```

base toml file

```toml
auth_email=examle@email.com
auth_key=exampleKey
record_id=exampleId
zone_identifier=exampleZoneIdentifier
record_name=exampleRecordName
```