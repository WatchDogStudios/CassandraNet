# CassandraNet

![Cassandra Logo](data/images/branding/CASS_LOGO_ICON_BLACK_SMALL.png)

A Rust powered server management platform for games. Used internally at WD Studios.

## What is this really?

CassandraNet simply just manages multiple servers (Virtual Machines, etc...) in one organized portal & API.

It helps server owners have a unified portal for all of their server needs, like connecting with authenticated users, verifying user sessions, banning players, managing server load, and so much more.

CassandraNet also gives you the ability to add "Plugins/Add-ons" to your instance to essentially fulfill more needs for your use case.

## How would i Intergrate this within my own systems?

For example, if you use Azure Playfab for managing your games servers, ever since they [Sunset Legacy Multiplayer Servers](https://community.playfab.com/questions/58173/i-wanted-to-host-custom-dedicated-servers-not-on-a.html), You can't connect non-azure servers to Playfab anymore, so you are left with no other option. 

What we recommend, is that you use Services like Playfab if you want a full live-ops solution right off the bat. 

If you want a more managed solution with more control, CassandraNet is the way to go.

## Code-of-conduct

CassandraNet follows the [Rust Lang's Code-of-Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Contributing

Feel free to submit a PR at anytime. we embrace open-source, and would love to grow this project with the OSS Community.

## Third-Party libraries Used

Check [license.html](CassandraNet/license.html) for the Libraries used in the project.