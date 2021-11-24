2.0.1
=====

* Fix imports to make no_std mode actually work.

2.0.0
=====

* Rework the Error interfaces to make it easier to differentiate
  between errors related to bad input and errors related to incorrect
  usage.

1.1.0
=====

* Add is_last_quintet_valid() and is_last_octet_valid() functions which can
  be used to figure out if the final input symbol is valid given the number
  of bits. This can be useful for finding and correcting errors.
