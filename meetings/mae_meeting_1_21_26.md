# meeting

## agenda

- how to populate functions table across all crates used? (+ all functions used)
    - in our prototype we also store function _signatures_ for the purpose of
      distinguishing indirect calls
    - but signatures were not used for dynamic dispatches
    - how do indirect function calls fit into our story? is this something we
      wanted to address, or was this just an impl step on the way to
      implementing verifopt for dynamic dispatches?

    - currently have two possible routes
        1. an interp pass that collects function info (would likely _not_ be
           able to get signatures from this directly, although maybe once we
           have function operands or something we can query something else to
           get sigs? but this is unknown)
        2. eagerly get function info + maybe signatures from a CrateMetadata
           struct
           - would have to figure out how to run rustc_driver stuff with a
             modified rustc, which i have yet been unable to do

