# RequiM Requirements

## 1. Something

## 2. Functional Requirements

### 2.1. Projects

#### 2.1.1. Unique project names

**ID:** REQ-F7
**Status:** In progress

Project names must be unique (case-insensitive)

### 2.2. List projects

The user should be able

#### 2.2.1 CLI invocation

**ID:** REQ-F1
**Status:** Fulfilled

Project listing shall be invoked with the subcommand "projects" without any
flags or further subcommands.

    requim projects

#### 2.2.2 Display of empty list

**ID:** REQ-F2
**Status:** Fulfilled

If there are no projects, the output of the subcommand shall be

    > requim projects
    You have no projects

### 2.3 Add project

#### 2.3.1 CLI incovation

**ID:** REQ-F3
**Status:** In progress

Adding a projects via the CLI is invoked with the subcommands "project new".

    requim project new "My Project"

### 2.3.2 CLI output

**ID:** REQ-F4
**Status:** In progress

#### 2.3.2.1 On success

**ID:** REQ-F5
**Status:** In progress

The output shall be

    > requim project new "My Project"
    Created project "My Project"

#### 2.3.2.2 On name collision

**ID:** REQ-F6
**Status:** In progress

In case of a name collision, the ouput shall be

    > requim project new "My Project"
    A project named "My Project" already exists

The output shall be printed to stderr in red
