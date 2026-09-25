"""
The binding-level contracts that are easy to regress and invisible in the examples:
the panic boundary, the container protocol of ``QueryResults``, and the bounding of
``node_neighborhood``.
"""

import pytest

from pyrudof import (
    InternalError,
    NeighborArc,
    QueryResults,
    RDFFormat,
    Rudof,
    RudofError,
)

# `:s :p :o ~ :ann` expands to `:ann rdf:reifies <<( :s :p :o )>>`, so `:ann`'s outgoing arc
# has an RDF 1.2 triple term as its neighbor — the term `rudof_rdf` cannot format yet.
REIFIED = "@prefix : <http://ex.org/> .\n:s :p :o ~ :ann .\n"
TWO_TRIPLES = "@prefix : <http://ex.org/> .\n:a :p :b .\n:c :p :d .\n"

ASK_TRUE = "ASK { <http://ex.org/a> <http://ex.org/p> <http://ex.org/b> }"
ASK_FALSE = "ASK { <http://ex.org/zz> <http://ex.org/p> <http://ex.org/b> }"
SELECT_TWO = "SELECT ?s ?o WHERE { ?s <http://ex.org/p> ?o }"
SELECT_NONE = "SELECT ?s WHERE { ?s <http://ex.org/absent> ?o }"
CONSTRUCT_TWO = "CONSTRUCT { ?s <http://ex.org/q> ?o } WHERE { ?s <http://ex.org/p> ?o }"
CONSTRUCT_NONE = "CONSTRUCT { ?s <http://ex.org/q> ?o } WHERE { ?s <http://ex.org/absent> ?o }"


def run(data: str, query: str) -> QueryResults:
    rudof = Rudof()
    rudof.read_data(input=data, format=RDFFormat.Turtle)
    rudof.read_query(query)
    return rudof.run_query()


def unrenderable_arc() -> NeighborArc:
    """The one arc of `:ann`, whose neighbor is a triple term that cannot be formatted."""
    rudof = Rudof()
    rudof.read_data(input=REIFIED, format=RDFFormat.Turtle)
    (arc,) = rudof.node_neighborhood(":ann", mode="outgoing")
    return arc


# --------------------------------------------------------------------------------------
# The panic boundary
# --------------------------------------------------------------------------------------


class TestPanicBoundary:
    """A Rust panic must arrive as a `RudofError`, not as pyo3's `PanicException`.

    `PanicException` derives from `BaseException`, so it escapes both `except RudofError`
    and `except Exception` and takes an embedding server down with it.
    """

    def test_internal_error_is_in_the_hierarchy(self) -> None:
        assert issubclass(InternalError, RudofError)
        assert issubclass(InternalError, Exception)

    def test_unimplemented_input_format_raises_instead_of_panicking(self) -> None:
        # `merge_from_reader` resolves N3 and TriG with `todo!()`.
        for unimplemented in (RDFFormat.N3, RDFFormat.TriG):
            with pytest.raises(InternalError):
                Rudof().read_data(input=TWO_TRIPLES, format=unimplemented)

    def test_unimplemented_query_type_raises_instead_of_panicking(self) -> None:
        rudof = Rudof()
        rudof.read_data(input=TWO_TRIPLES, format=RDFFormat.Turtle)
        rudof.read_query("DESCRIBE <http://ex.org/a>")
        with pytest.raises(InternalError):
            rudof.run_query()

    def test_a_panic_is_caught_by_except_rudof_error(self) -> None:
        with pytest.raises(RudofError):
            _ = unrenderable_arc().neighbor

    def test_a_panic_is_caught_by_except_exception(self) -> None:
        # The regression this guards: `except Exception` used to miss the panic entirely.
        try:
            _ = unrenderable_arc().neighbor
        except Exception as caught:  # noqa: BLE001 - that it is an Exception is the point
            assert isinstance(caught, InternalError)
        else:
            pytest.fail("reading an unrenderable term should have raised")

    def test_the_panic_message_survives(self) -> None:
        with pytest.raises(InternalError, match="not yet implemented"):
            _ = unrenderable_arc().neighbor

    def test_repr_degrades_rather_than_raising(self) -> None:
        # `repr` is what logging and debuggers call, so it has to work even when a term
        # does not render.
        rendered = repr(unrenderable_arc())
        assert "<unrenderable>" in rendered
        assert "rdf-syntax-ns#reifies" in rendered

    def test_renderable_terms_are_unaffected(self) -> None:
        rudof = Rudof()
        rudof.read_data(input=TWO_TRIPLES, format=RDFFormat.Turtle)
        (arc,) = rudof.node_neighborhood(":a", mode="outgoing")
        assert arc.node == "http://ex.org/a"
        assert arc.neighbor == "http://ex.org/b"
        assert arc.root == "http://ex.org/a"
        assert "<unrenderable>" not in repr(arc)


# --------------------------------------------------------------------------------------
# The QueryResults container protocol
# --------------------------------------------------------------------------------------


class TestQueryResultsContainerProtocol:
    """`len()` and iteration must agree, and `bool()` must work for every query shape.

    They used to contradict each other: an ASK reported `len() == 1` and iterated zero
    items, and a CONSTRUCT over two triples reported `len() == 0`.
    """

    @pytest.mark.parametrize(
        ("query", "expected"),
        [(SELECT_TWO, 2), (SELECT_NONE, 0)],
        ids=["two-solutions", "no-solutions"],
    )
    def test_select_length_matches_iteration(self, query: str, expected: int) -> None:
        results = run(TWO_TRIPLES, query)
        assert len(results) == expected
        assert len(list(results)) == expected
        assert bool(results) is (expected > 0)

    @pytest.mark.parametrize("query", [ASK_TRUE, ASK_FALSE], ids=["true", "false"])
    def test_ask_refuses_the_container_protocol(self, query: str) -> None:
        results = run(TWO_TRIPLES, query)
        with pytest.raises(TypeError, match="ASK result has no length"):
            len(results)
        with pytest.raises(TypeError, match="ASK result is not iterable"):
            iter(results)

    @pytest.mark.parametrize(
        ("query", "answer"),
        [(ASK_TRUE, True), (ASK_FALSE, False)],
        ids=["true", "false"],
    )
    def test_ask_answers_through_bool(self, query: str, answer: bool) -> None:
        results = run(TWO_TRIPLES, query)
        assert results.boolean is answer
        # `bool()` must not fall through to the raising `__len__`.
        assert bool(results) is answer

    @pytest.mark.parametrize(
        "query", [CONSTRUCT_TWO, CONSTRUCT_NONE], ids=["two-triples", "no-triples"]
    )
    def test_construct_refuses_the_container_protocol(self, query: str) -> None:
        results = run(TWO_TRIPLES, query)
        with pytest.raises(TypeError, match="CONSTRUCT or DESCRIBE result has no length"):
            len(results)
        with pytest.raises(TypeError, match="CONSTRUCT or DESCRIBE result is not iterable"):
            iter(results)

    @pytest.mark.parametrize(
        ("query", "nonempty"),
        [(CONSTRUCT_TWO, True), (CONSTRUCT_NONE, False)],
        ids=["two-triples", "no-triples"],
    )
    def test_construct_emptiness_through_bool(self, query: str, nonempty: bool) -> None:
        results = run(TWO_TRIPLES, query)
        assert results.is_graph
        assert bool(results) is nonempty

    def test_construct_graph_is_still_reachable(self) -> None:
        # The graph stays available as a string; only the container protocol is refused.
        results = run(TWO_TRIPLES, CONSTRUCT_TWO)
        graph = results.graph
        assert graph is not None
        assert "http://ex.org/q" in graph

        # And it round-trips into another session, which is how it is meant to be consumed.
        reloaded = Rudof()
        reloaded.read_data(input=graph, format=RDFFormat.Turtle)
        reloaded.read_query("SELECT ?s WHERE { ?s <http://ex.org/q> ?o }")
        assert len(reloaded.run_query()) == 2


# --------------------------------------------------------------------------------------
# Bounding node_neighborhood
# --------------------------------------------------------------------------------------

FANOUT = 50
HUB = (
    "@prefix : <http://ex.org/> .\n:hub "
    + " ; ".join(f":p{i} :o{i}" for i in range(FANOUT))
    + " .\n"
)


class TestNeighborhoodLimit:
    """`node_neighborhood` materializes its arcs, so it needs a way to bound the work."""

    @pytest.fixture
    def rudof(self) -> Rudof:
        rudof = Rudof()
        rudof.read_data(input=HUB, format=RDFFormat.Turtle)
        return rudof

    def test_no_limit_returns_the_whole_fanout(self, rudof: Rudof) -> None:
        arcs = rudof.node_neighborhood(":hub", mode="outgoing")
        assert len(list(arcs)) == FANOUT
        assert arcs.truncated is False

    @pytest.mark.parametrize("limit", [0, 1, 10, FANOUT - 1])
    def test_limit_bounds_the_arcs_and_reports_truncation(self, rudof: Rudof, limit: int) -> None:
        arcs = rudof.node_neighborhood(":hub", mode="outgoing", limit=limit)
        assert arcs.truncated is True
        assert len(list(arcs)) == limit

    @pytest.mark.parametrize("limit", [FANOUT, FANOUT + 1, 10_000])
    def test_a_limit_at_or_above_the_fanout_does_not_truncate(self, rudof: Rudof, limit: int) -> None:
        # The off-by-one that matters: exactly `FANOUT` arcs is not a truncation.
        arcs = rudof.node_neighborhood(":hub", mode="outgoing", limit=limit)
        assert arcs.truncated is False
        assert len(list(arcs)) == FANOUT

    def test_length_hint_is_exact_and_shrinks(self, rudof: Rudof) -> None:
        arcs = rudof.node_neighborhood(":hub", mode="outgoing", limit=10)
        assert arcs.__length_hint__() == 10
        next(arcs)
        assert arcs.__length_hint__() == 9
        assert len(list(arcs)) == 9
        assert arcs.__length_hint__() == 0
