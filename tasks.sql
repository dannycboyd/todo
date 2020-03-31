--
-- PostgreSQL database dump
--

-- Dumped from database version 11.5
-- Dumped by pg_dump version 11.5

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: tasks; Type: TABLE; Schema: public; Owner: dannyboyd
--

CREATE TABLE public.tasks (
    id integer NOT NULL,
    start date,
    repeats character varying(7),
    title character varying(80),
    note character varying(1000),
    finished boolean
);


ALTER TABLE public.tasks OWNER TO dannyboyd;

--
-- Name: tasks_id_seq; Type: SEQUENCE; Schema: public; Owner: dannyboyd
--

CREATE SEQUENCE public.tasks_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.tasks_id_seq OWNER TO dannyboyd;

--
-- Name: tasks_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: dannyboyd
--

ALTER SEQUENCE public.tasks_id_seq OWNED BY public.tasks.id;


--
-- Name: tasks id; Type: DEFAULT; Schema: public; Owner: dannyboyd
--

ALTER TABLE ONLY public.tasks ALTER COLUMN id SET DEFAULT nextval('public.tasks_id_seq'::regclass);


--
-- Data for Name: tasks; Type: TABLE DATA; Schema: public; Owner: dannyboyd
--

COPY public.tasks (id, start, repeats, title, note, finished) FROM stdin;
1	1995-10-16	y	My Birthday	It is my date of birth!	f
\.


--
-- Name: tasks_id_seq; Type: SEQUENCE SET; Schema: public; Owner: dannyboyd
--

SELECT pg_catalog.setval('public.tasks_id_seq', 1, true);


--
-- PostgreSQL database dump complete
--

