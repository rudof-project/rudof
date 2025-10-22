from pyrudof import Rudof, RudofConfig, RDFFormat, QueryResultFormat 

endpoint = "https://plod.dbcls.jp/repositories/RDFPortal_VoID"

sparql_query = """
PREFIX void: <http://rdfs.org/ns/void#>
PREFIX sd: <http://www.w3.org/ns/sparql-service-description#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

CONSTRUCT WHERE {
  [
    a sd:Service ;
    sd:defaultDataset [
       a sd:Dataset ;
       sd:namedGraph [
         sd:name <http://sparql.uniprot.org/uniprot> ;
         a sd:NamedGraph ;
         sd:endpoint ?ep_url ;
         sd:graph [
           a void:Dataset ;
           void:triples ?total_count ;
           void:classes ?class_count ;
           void:properties ?property_count ;
           void:distinctObjects ?uniq_object_count ;
           void:distinctSubjects ?uniq_subject_count ;
           void:classPartition [
             void:class ?class_name ;
             void:entities ?class_triple_count
           ] ;
           void:propertyPartition [
             void:property ?property_name ;
             void:triples ?property_triple_count
           ]
         ]
       ]
     ]
  ] .
}
"""
rudof = Rudof(RudofConfig())
rudof.use_endpoint(endpoint)

result = rudof.run_query_construct_str(sparql_query, QueryResultFormat.Turtle)

print(result)