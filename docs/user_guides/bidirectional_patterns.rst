Bidirectional Patterns
======================

`BidirectionalPattern` enables round-trip parsing: parse a string, modify the extracted
values, and format them back while maintaining the original format constraints.

Basic Usage
-----------

Create a `BidirectionalPattern` and parse a string:

.. doctest::

   >>> from formatparse import BidirectionalPattern
   >>> formatter = BidirectionalPattern("{name:>10}: {value:05d}")
   >>> result = formatter.parse("      John: 00042")
   >>> result.named['name']
   'John'
   >>> result.named['value']
   42

Formatting Back
---------------

The `BidirectionalResult` object can format itself back using the original pattern:

.. doctest::

   >>> result.format()
   '      John: 00042'

Modifying Values
----------------

You can modify the extracted values and format them back:

.. doctest::

   >>> result.named['value'] = 100
   >>> result.format()
   '      John: 00100'
   
   >>> result.named['name'] = "Alice"
   >>> result.format()
   '     Alice: 00100'

Validation
----------

Validate values against the pattern's constraints:

.. doctest::

   >>> result.named['value'] = 42
   >>> is_valid, errors = result.validate()
   >>> is_valid
   True
   >>> errors
   []
   
   >>> result.named['value'] = "not a number"
   >>> is_valid, errors = result.validate()
   >>> is_valid
   False
   >>> len(errors) > 0
   True

Using format() Method Directly
-------------------------------

You can also use the pattern's `format()` method directly:

.. doctest::

   >>> formatted = formatter.format({"name": "Alice", "value": 42})
   >>> formatted.startswith("     Alice")
   True
   >>> "00042" in formatted
   True

Real-World Use Cases
--------------------

Configuration File Updates
~~~~~~~~~~~~~~~~~~~~~~~~~~

Parse configuration files, modify values, and write them back:

.. testcode::

   formatter = BidirectionalPattern("PORT={port:05d}")
   result = formatter.parse("PORT=00080")
   result.named['port'] = 8080
   new_config = result.format()
   print(new_config)

.. testoutput::

   PORT=08080

Data Transformation Pipelines
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Transform data while maintaining format constraints:

.. doctest::

   >>> formatter = BidirectionalPattern("ID:{id:05d} Name:{name:>10}")
   >>> result = formatter.parse("ID:00042 Name:      John")
   >>> result.named['id']
   42
   >>> result.named['id'] = 100
   >>> result.named['name'] = "Alice"
   >>> formatted = result.format()
   >>> formatted.startswith("ID:00100 Name:")
   True
   >>> formatted.endswith("Alice")
   True

Form Validation
~~~~~~~~~~~~~~~

Validate user input against expected formats:

.. testcode::

   formatter = BidirectionalPattern("{username:>10}: {score:05d}")
   user_input = "     alice: 00100"
   result = formatter.parse(user_input)
   
   if result:
       is_valid, errors = result.validate()
       if is_valid:
           print(f"Valid input: {result.format()}")
       else:
           print(f"Validation errors: {errors}")
   else:
       print("Input does not match pattern")

.. testoutput::

   Valid input:      alice: 00100

