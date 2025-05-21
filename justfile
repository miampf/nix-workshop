set shell := ["bash", "-uc"]

[group("presentation")]
preview:
    marp -p presentation/slides.md
