# Omoide (思い出)

Browse your tweets and delete them.

## Setup

```
cp .env.sample .env
$EDITOR .env  # set your auth information
```

## Build

```
make
```

## Use omoide

```
omoide
```

If you want to delete tweets,

```
omoide -delete
```

*Deleting tweets cannot be reverted!*

