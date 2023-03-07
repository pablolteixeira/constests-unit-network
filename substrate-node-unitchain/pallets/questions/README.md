# Questions Module

A simple, secure module for interact with question and answer.

## Overview

The Questions module provides functionality for question and answer that use asset id.

### Goals

The questions module use Substrate is designed to make the following possible:

* Ask question with details.
* Reply question with details.
* Vote answers for with particular token regrading to question's token.

## Interface

### Dispatchable Functions

* `ask_question` - Create new question with details.
* `reply_question` - Answer a `question` with details.
* `vote` - Vote answer of question.

### Public Functions

* `question_id` - Get the amount of total questions.
* `questions_data` - Get the question data from `id` of `question`.
* `answers_data` - Get the answer data from `id` of `question`.
* `user_vote_data` - Get the user vote data from `id` of `user` and `id` of `question`.

### Config Modules

* `Fungibles` - Assets Pallet for each question.
* `MaxLength` - Maximum of bytes of each details.
* `MinAmountToAsk` - Minimum to create questions and reply.


### Prerequisites

Import the Assets module and specify constant parameter of Module.

## File Structure

### libs.rs

* Pallet Logic include extrinsic.

### types.rs

* Storage types that use in pallet.

### mock.rs

* Mock frame and runtime that use in test.

### test.rs

* All tests cover in runtime test.

License: Unlicense