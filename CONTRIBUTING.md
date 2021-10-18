# Contributing

## Raise an Issue

Raising [issues](https://github.com/getsynth/synth/issues) is encouraged. We have some templates to help you get started.

## Running Locally

To compile from source, see the `Compiling from source` tab in the [docs](https://getsynth.github.io/synth/getting_started/installation).

## Running Tests

Synth has reasonable test coverage - and we are working on improving this 
every day. We encourage PRs to come with tests. If you're not sure about 
what a test should look like, feel free to get in touch.

To run the test suite - just run `cargo test` at the root of the repository.

## Committing

We use the [Angular Commit Guidelines](https://github.com/angular/angular/blob/master/CONTRIBUTING.md#commit). We expect all commits to conform to these guidelines.

Furthermore, commits should be squashed before being merged to master.

Also, make sure your commits don't trigger any warnings from Clippy by running: `cargo clippy --tests --all-targets`. If you have a good reason to contradict Clippy, insert an #allow[] macro, so that it won't complain.

Plus, make sure your commits are properly formatted. You can automate this 
process by copying over the `pre-commit` file in `tools/hooks/pre-commit` to `.git/hooks/pre-commit` which will make git automatically format your code before committing them.
