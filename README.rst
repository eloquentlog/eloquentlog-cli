Eloquentlog CLI
===============

.. image:: https://gitlab.com/eloquentlog/eloquentlog-cli/badges/master/pipeline.svg
   :target: https://gitlab.com/eloquentlog/eloquentlog-cli/commits/master

.. image:: https://gitlab.com/eloquentlog/eloquentlog-cli/badges/master/coverage.svg
   :target: https://gitlab.com/eloquentlog/eloquentlog-cli/commits/master

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

   # check code using all vet:xxx targets
   % make vet:all

Test
~~~~

.. code:: zsh

   % make test

Coverage
~~~~~~~~

``cov`` requires kcov.


.. code:: zsh

   # (optional)
   % .tools/setup-kcov

   % make coverage

CI
~~

Run CI jobs on local docker conatiner (Gentoo Linux) using gitlab-runner.
See `.gitlab-ci.yml`.


.. code:: zsh

   # install gitlab-runner into .tools
   % .tools/setup-gitlab-runner

   # prepare environment variables for CI via .env.ci
   % cp .env.ci.sample .env

   # e.g. test (see .gitlab-ci.yml)
   % .tools/ci-runner test


License
-------

.. code:: text

   ┏━╸╻  ┏━┓┏━┓╻ ╻┏━╸┏┓╻╺┳╸╻  ┏━┓┏━╸
   ┣╸ ┃  ┃ ┃┃┓┃┃ ┃┣╸ ┃┗┫ ┃ ┃  ┃ ┃┃╺┓
   ┗━╸┗━╸┗━┛┗┻┛┗━┛┗━╸╹ ╹ ╹ ┗━╸┗━┛┗━┛

   CLI
   Copyright 2019 Lupine Software LLC


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