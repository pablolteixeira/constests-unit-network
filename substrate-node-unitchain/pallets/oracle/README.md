# Oracle Module

## Overview

The Oracle module provides feeders can feed any data into pallet and client or another pallet can use it.

### Goals

The oracle module use Substrate is designed to make the following possible:

* Feeders feed any data with key and value.
* Client get any data via key and value.

## Interface

### Dispatchable Functions

* `feed_values` - Feeder feed any data.
* `add_oracle_key` - Register new oracle key.

### Public Functions

* `get_raw_values` - Get raw value of oracle key that feeder feeded.
* `get` - Get value of oracle key that aggregated.
* `key_details` - Get detail of oracle key.

### Config Modules

* `CombineData` - A function that aggregate data when more that 1 value.
* `OracleKey` - Oracle key type.
* `OracleValue` - Oracle value type.
* `Members` - A group of feeders.
* `Time` - Time providers use in feed data.


### Prerequisites

1. Specify constant parameter of Module.
2. Genesis config of oracle key details.

## File Structure

### lib.rs

* Pallet Logic include extrinsic.

### mock.rs

* Mock frame and runtime that use in test.

### test.rs

* All tests cover in runtime test.

### combine_data.rs

* Aggregate data with median operation.

License: Unlicense