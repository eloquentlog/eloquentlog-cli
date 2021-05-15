Eloquentlog CLI
===============

.. image:: https://gitlab.com/eloquentlog/eloquentlog-cli/badges/trunk/pipeline.svg
   :target: https://gitlab.com/eloquentlog/eloquentlog-cli/commits/trunk

.. image:: https://gitlab.com/eloquentlog/eloquentlog-cli/badges/trunk/coverage.svg
   :target: https://gitlab.com/eloquentlog/eloquentlog-cli/commits/trunk

.. code:: text

   Eloquentlog

   ╔═╗╦  ╦
   ║  ║  ║
   ╚═╝╩═╝╩

The command line interface of Eloquentlog_.


Repository
----------

https://gitlab.com/eloquentlog/eloquentlog-cli


Installation
------------

TODO

.. code:: zsh

   % cargo install eloquentlog-cli

   # or clone and build
   % git clone https://github.com/eloquentlog/eloquentlog-cli.git && \
     cd eloquentlog-cli
   % make install


Usage
-----

TODO


Build
-----

Check ``make help``

.. code:: zsh

   # debug build
   % make build:debug


Development
-----------

Vet
~~~

.. code:: zsh

   # check code using all verify targets
   % make verify:all

Test
~~~~

.. code:: zsh

   % make test

Coverage
~~~~~~~~

``cov`` requires kcov.


.. code:: zsh

   % make coverage:all

CI
~~

You can run any CI job a on local docker conatiner (Gentoo Linux) by using
gitlab-runner. See `.gitlab-ci.yml`.


.. code:: zsh

   # prepare environment variables for CI via .env.ci
   % cp .env.ci.sample .env

   # e.g. test (see Makefile)
   % make runner-test


License
-------

.. code:: text

   ┏━╸╻  ┏━┓┏━┓╻ ╻┏━╸┏┓╻╺┳╸╻  ┏━┓┏━╸
   ┣╸ ┃  ┃ ┃┃┓┃┃ ┃┣╸ ┃┗┫ ┃ ┃  ┃ ┃┃╺┓
   ┗━╸┗━╸┗━┛┗┻┛┗━┛┗━╸╹ ╹ ╹ ┗━╸┗━┛┗━┛

   CLI
   Copyright 2019-2021 Lupine Software LLC


``GPL-3.0``.

.. code:: text

   This is free software: You can redistribute it and/or modify
   it under the terms of the GNU General Public License as
   published by the Free Software Foundation, either version 3 of the
   License, or (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
   GNU Affero General Public License for more details.

   You should have received a copy of the GNU Affero General Public License
   along with this program. If not, see <https://www.gnu.org/licenses/>.

.. _Eloquentlog: https://eloquentlog.com/
