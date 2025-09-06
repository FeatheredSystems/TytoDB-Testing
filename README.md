# TytoDB Testing routine

This repository keeps the codebase used to simulate a client and perform a testing routine, which is run on GitHub Actions in the main repo and on my local machine during the development phase.

The test works by running 4 operations and covering all their branches.
- Insert: Run the insert operation
- Edit: Delete the existing row and insert another with the changes. In practice, it runs *Insert* and *Delete*
- Delete: Perform deletes.

All the mentioned operations test MVCC, Graveyard, Index, HashMap, ds_cache, and burning map; Only the *Edit* and *Delete* test the query operations altogether. After executing the listed operations, queries are run, testing the scan query — Testing the indexed query isn't necessary since *Edit* and *Delete* do it.

To run this routine yourself, you must clone this repo on your machine and make sure your TytoDB instance is up and running, having its dir at "$HOME/TytoDB"; if that's not the case, you must alter this in the codebase. The port and host are TytoDB's defaults, so if you changed them, you must alter this detail in the codebase before running this testing routine.

The quality assurance codebase is currently lean, proportionally as the current system error surface. If this code returns an error resulting from either the database or the routine, feel welcome to create an issue, discussion, or send me an email at the address "tytodatabase@gmail.com".

