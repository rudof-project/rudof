# Show information of RDF data
rudof data user.ttl

# Show information of RDF data in a different format
rudof data user.ttl -r rdfxml

# Show information about a node in RDF data
rudof node -n :a user.ttl

# Show information about a node in RDF data behind a SPARQL endpoint
rudof node -n wd:Q80 -e wikidata

# Show information about a node in RDF data (incoming edges)
rudof node -n :a user.ttl -m incoming
