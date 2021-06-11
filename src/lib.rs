/* 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright 2021 Robert D. French
 */
//! PortunusD - The Daemon of Ports and Doors
//!
//! PortunusD is a network application server, inspired by relayd and inetd, which aims to ease the
//! scaling of single-threaded request/response-style applications: web applications, DNS queries,
//! etc. PortunusD allows applications to embrace the "serverless" style of development, but without
//! throwing away all the luxuries of the operating system.


pub mod door;
pub mod illumos;
pub mod jamb;
pub mod server_procedure;
pub mod application_doorway;
