--
-- PostgreSQL database dump
--

-- Dumped from database version 11.7
-- Dumped by pg_dump version 11.7

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

--
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: tfaadmin
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO tfaadmin;

--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: tfaadmin
--

CREATE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO tfaadmin;

SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO tfaadmin;

--
-- Name: blacklists; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.blacklists (
    bid bigint NOT NULL,
    uuid_a bigint NOT NULL,
    uuid_b bigint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.blacklists OWNER TO tfaadmin;

--
-- Name: chat_groups; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.chat_groups (
    gid bigint NOT NULL,
    group_name character varying(500) NOT NULL,
    group_thumbnail character varying(500) NOT NULL,
    uuid bigint NOT NULL,
    person_count smallint DEFAULT 0 NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.chat_groups OWNER TO tfaadmin;

--
-- Name: chat_groups_uids; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.chat_groups_uids (
    guid bigint NOT NULL,
    gid bigint NOT NULL,
    uuid bigint NOT NULL,
    latest_timestamp bigint NOT NULL,
    unread_count smallint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.chat_groups_uids OWNER TO tfaadmin;

--
-- Name: chat_messages; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.chat_messages (
    mid bigint NOT NULL,
    send_id bigint NOT NULL,
    to_id bigint NOT NULL,
    content character varying(500) NOT NULL,
    created_timestamp bigint NOT NULL,
    kind smallint DEFAULT 0 NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.chat_messages OWNER TO tfaadmin;

--
-- Name: enemys; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.enemys (
    eid bigint NOT NULL,
    enemy_name character varying(200) NOT NULL,
    model_path character varying(100) NOT NULL,
    thumbnail character varying(200) NOT NULL,
    max_hp integer NOT NULL,
    attack_power integer NOT NULL,
    move_speed real NOT NULL,
    max_mana integer NOT NULL,
    defense integer NOT NULL,
    animation_hit_delay real NOT NULL,
    spawn_style_class character varying(200) NOT NULL,
    bp_enemy character varying(200) NOT NULL,
    ap_enemy character varying(200) NOT NULL,
    skm_enemy character varying(200) NOT NULL,
    aenemy_die character varying(200) NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.enemys OWNER TO tfaadmin;

--
-- Name: equipment_kinds; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.equipment_kinds (
    kid bigint NOT NULL,
    name character varying(200) NOT NULL,
    kind smallint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.equipment_kinds OWNER TO tfaadmin;

--
-- Name: equipments; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.equipments (
    eid bigint NOT NULL,
    kid bigint NOT NULL,
    name character varying(200) NOT NULL,
    thumbnail character varying(200) NOT NULL,
    price integer NOT NULL,
    hp integer NOT NULL,
    multiplier real NOT NULL,
    kind smallint NOT NULL,
    is_default smallint DEFAULT 1 NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.equipments OWNER TO tfaadmin;

--
-- Name: friends; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.friends (
    fid bigint NOT NULL,
    uuid_a bigint NOT NULL,
    uuid_b bigint NOT NULL,
    state smallint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.friends OWNER TO tfaadmin;

--
-- Name: gem_relateds; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.gem_relateds (
    grid bigint NOT NULL,
    obj_id bigint NOT NULL,
    gid bigint NOT NULL,
    obj_type smallint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.gem_relateds OWNER TO tfaadmin;

--
-- Name: gems; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.gems (
    gid bigint NOT NULL,
    gem_icon character varying(200) NOT NULL,
    gem_selected_material character varying(200) NOT NULL,
    gem_link_material character varying(200) NOT NULL,
    model_path character varying(200) NOT NULL,
    kind smallint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.gems OWNER TO tfaadmin;

--
-- Name: link_accounts; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.link_accounts (
    lid bigint NOT NULL,
    uuid bigint NOT NULL,
    account_type smallint DEFAULT 0 NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.link_accounts OWNER TO postgres;

--
-- Name: mall_first_recharge_gift_package_assets; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.mall_first_recharge_gift_package_assets (
    id bigint NOT NULL,
    obj_id bigint NOT NULL,
    obj_type smallint NOT NULL,
    sort_value smallint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mall_first_recharge_gift_package_assets OWNER TO tfaadmin;

--
-- Name: mall_first_recharge_gift_packages; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.mall_first_recharge_gift_packages (
    id bigint NOT NULL,
    name character varying(50) NOT NULL,
    bg_url character varying(200) NOT NULL,
    bg_desc character varying(200) NOT NULL,
    asset_desc character varying(200) NOT NULL,
    price real NOT NULL,
    product_number character varying(200) NOT NULL,
    status smallint DEFAULT 0 NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mall_first_recharge_gift_packages OWNER TO tfaadmin;

--
-- Name: mall_gem_stores; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.mall_gem_stores (
    id bigint NOT NULL,
    name character varying(200) NOT NULL,
    gem_count integer NOT NULL,
    first_buy_handsel integer NOT NULL,
    late_buy_handsel integer NOT NULL,
    price real NOT NULL,
    product_number character varying(200) NOT NULL,
    status smallint DEFAULT 0 NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mall_gem_stores OWNER TO tfaadmin;

--
-- Name: mall_supply_station_assets; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.mall_supply_station_assets (
    id bigint NOT NULL,
    sid bigint NOT NULL,
    obj_id bigint NOT NULL,
    obj_type smallint NOT NULL,
    sort_value smallint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mall_supply_station_assets OWNER TO tfaadmin;

--
-- Name: mall_supply_stations; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.mall_supply_stations (
    id bigint NOT NULL,
    name character varying(200) NOT NULL,
    description character varying(200) NOT NULL,
    thumbnail character varying(200) NOT NULL,
    gem_count integer NOT NULL,
    valid_period_day smallint NOT NULL,
    price real NOT NULL,
    product_number character varying(200) NOT NULL,
    status smallint DEFAULT 0 NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mall_supply_stations OWNER TO tfaadmin;

--
-- Name: mall_user_buy_gem_store_records; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.mall_user_buy_gem_store_records (
    rid bigint NOT NULL,
    uuid bigint NOT NULL,
    gsid bigint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mall_user_buy_gem_store_records OWNER TO tfaadmin;

--
-- Name: mall_user_buy_supply_station_records; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.mall_user_buy_supply_station_records (
    id bigint NOT NULL,
    uuid bigint NOT NULL,
    sid bigint NOT NULL,
    expire_time bigint NOT NULL,
    latest_receive_time bigint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mall_user_buy_supply_station_records OWNER TO tfaadmin;

--
-- Name: mall_user_first_recharge_gift_package_records; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.mall_user_first_recharge_gift_package_records (
    id bigint NOT NULL,
    uuid bigint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.mall_user_first_recharge_gift_package_records OWNER TO tfaadmin;

--
-- Name: player_mount_equipments; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.player_mount_equipments (
    id bigint NOT NULL,
    pid bigint NOT NULL,
    uid bigint NOT NULL,
    equipment_id bigint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.player_mount_equipments OWNER TO tfaadmin;

--
-- Name: players; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.players (
    pid bigint NOT NULL,
    player_name character varying(200) NOT NULL,
    model_path character varying(100) NOT NULL,
    thumbnail character varying(200) NOT NULL,
    max_hp integer NOT NULL,
    attack_power integer NOT NULL,
    move_speed real NOT NULL,
    max_mana integer NOT NULL,
    defense integer NOT NULL,
    animation_hit_delay real NOT NULL,
    spawn_style_class character varying(200) NOT NULL,
    level smallint NOT NULL,
    star_level smallint NOT NULL,
    level_experience integer NOT NULL,
    is_default smallint DEFAULT 1 NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.players OWNER TO tfaadmin;

--
-- Name: props_resources_categorys; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.props_resources_categorys (
    rid bigint NOT NULL,
    name character varying(200) NOT NULL,
    count integer NOT NULL,
    thumbnail character varying(200) NOT NULL,
    description character varying(200) NOT NULL,
    kind smallint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.props_resources_categorys OWNER TO tfaadmin;

--
-- Name: props_speed_up_categorys; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.props_speed_up_categorys (
    sid bigint NOT NULL,
    name character varying(200) NOT NULL,
    thumbnail character varying(200) NOT NULL,
    description character varying(200) NOT NULL,
    speed_time bigint NOT NULL,
    count integer NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.props_speed_up_categorys OWNER TO tfaadmin;

--
-- Name: purchase_orders; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.purchase_orders (
    oid bigint NOT NULL,
    obj_id bigint NOT NULL,
    obj_type smallint NOT NULL,
    uuid bigint NOT NULL,
    hash character varying(200) NOT NULL,
    product_number character varying(200) NOT NULL,
    pay_platform smallint NOT NULL,
    order_no bigint NOT NULL,
    status smallint NOT NULL,
    pay_time bigint NOT NULL,
    price real NOT NULL,
    request_receipt_data text NOT NULL,
    response_receipt_data text NOT NULL,
    deleted_time bigint DEFAULT 0 NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.purchase_orders OWNER TO tfaadmin;

--
-- Name: server_lists; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.server_lists (
    slid bigint NOT NULL,
    name character varying(100) NOT NULL,
    country_code character varying(10) NOT NULL,
    area character varying(50) NOT NULL,
    ip character varying(200) NOT NULL,
    port smallint NOT NULL,
    server_type smallint NOT NULL,
    state smallint DEFAULT 0 NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.server_lists OWNER TO tfaadmin;

--
-- Name: servers; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.servers (
    sid bigint NOT NULL,
    server_number integer NOT NULL,
    name character varying(500) NOT NULL,
    ip character varying(500) NOT NULL,
    ports smallint NOT NULL,
    person_count integer NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.servers OWNER TO tfaadmin;

--
-- Name: skill_fight_relateds; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.skill_fight_relateds (
    id bigint NOT NULL,
    obj_id bigint NOT NULL,
    skill_id bigint NOT NULL,
    cool_down integer NOT NULL,
    attack_power integer NOT NULL,
    mana_power integer NOT NULL,
    probability smallint DEFAULT 0 NOT NULL,
    level smallint DEFAULT 0 NOT NULL,
    level_experience integer DEFAULT 0 NOT NULL,
    obj_type smallint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.skill_fight_relateds OWNER TO tfaadmin;

--
-- Name: skills; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.skills (
    id bigint NOT NULL,
    thumbnail character varying(200) NOT NULL,
    skill_name character varying(200) NOT NULL,
    skill_description character varying(200) NOT NULL,
    model_path character varying(200) NOT NULL,
    cool_down integer NOT NULL,
    attack_power integer NOT NULL,
    mana_power integer NOT NULL,
    level smallint DEFAULT 0 NOT NULL,
    level_experience integer DEFAULT 0 NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.skills OWNER TO tfaadmin;

--
-- Name: user_assets; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.user_assets (
    asid bigint NOT NULL,
    uid bigint NOT NULL,
    golds integer DEFAULT 0 NOT NULL,
    diamonds integer DEFAULT 0 NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.user_assets OWNER TO tfaadmin;

--
-- Name: user_chat_unread_counts; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.user_chat_unread_counts (
    ucid bigint NOT NULL,
    uuid_s bigint NOT NULL,
    uuid_d bigint NOT NULL,
    latest_timestamp bigint NOT NULL,
    unread_count smallint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.user_chat_unread_counts OWNER TO tfaadmin;

--
-- Name: user_equipments; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.user_equipments (
    id bigint NOT NULL,
    eid bigint NOT NULL,
    uid bigint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.user_equipments OWNER TO tfaadmin;

--
-- Name: user_player_tracks; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.user_player_tracks (
    tid bigint NOT NULL,
    pid bigint NOT NULL,
    uid bigint NOT NULL,
    rotaion_x real NOT NULL,
    rotaion_y real NOT NULL,
    rotaion_z real NOT NULL,
    location_x real NOT NULL,
    location_y real NOT NULL,
    location_z real NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.user_player_tracks OWNER TO tfaadmin;

--
-- Name: user_players; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.user_players (
    id bigint NOT NULL,
    pid bigint NOT NULL,
    uid bigint NOT NULL,
    max_hp integer NOT NULL,
    attack_power integer NOT NULL,
    move_speed real NOT NULL,
    max_mana integer NOT NULL,
    defense integer NOT NULL,
    level smallint NOT NULL,
    star_level smallint NOT NULL,
    level_experience integer NOT NULL,
    is_default smallint NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.user_players OWNER TO tfaadmin;

--
-- Name: users; Type: TABLE; Schema: public; Owner: tfaadmin
--

CREATE TABLE public.users (
    uuid bigint NOT NULL,
    uid integer NOT NULL,
    name character varying(500) NOT NULL,
    avatar character varying(500) NOT NULL,
    login_days integer NOT NULL,
    server_id integer NOT NULL,
    modify_time timestamp without time zone DEFAULT now() NOT NULL,
    created_time timestamp without time zone DEFAULT now() NOT NULL,
    action_force integer DEFAULT 1000 NOT NULL
);


ALTER TABLE public.users OWNER TO tfaadmin;

--
-- Name: users_login_days_seq; Type: SEQUENCE; Schema: public; Owner: tfaadmin
--

CREATE SEQUENCE public.users_login_days_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.users_login_days_seq OWNER TO tfaadmin;

--
-- Name: users_login_days_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tfaadmin
--

ALTER SEQUENCE public.users_login_days_seq OWNED BY public.users.login_days;


--
-- Name: users login_days; Type: DEFAULT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.users ALTER COLUMN login_days SET DEFAULT nextval('public.users_login_days_seq'::regclass);


--
-- Data for Name: __diesel_schema_migrations; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.__diesel_schema_migrations (version, run_on) FROM stdin;
00000000000000	2020-06-03 07:17:21.652143
\.


--
-- Data for Name: blacklists; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.blacklists (bid, uuid_a, uuid_b, modify_time, created_time) FROM stdin;
5074796791009632659	1273575576218516600	793426562737268076	2020-08-06 09:29:11.627358	2020-08-06 09:29:11.627358
4190319770962973076	5001370379009920352	6236962157120125250	2020-09-04 07:58:37.93199	2020-09-04 07:58:37.93199
\.


--
-- Data for Name: chat_groups; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.chat_groups (gid, group_name, group_thumbnail, uuid, person_count, modify_time, created_time) FROM stdin;
1389812494058547205	进扩,Governor10006381,Governor10006382		42184729541320523	3	2020-09-04 08:14:22.574295	2020-09-04 08:14:22.574295
1155267990789338481	进扩,Governor10006381,Governor10006424		5001370379009920352	3	2020-09-08 07:34:03.271074	2020-09-08 07:34:03.271074
1241873839936331673	我比较特别,Governor10006519,Governor10006521		3455115140489977330	3	2020-09-09 01:49:22.372215	2020-09-09 01:49:22.372215
763106957186478718	Governor10006556,Governor10006522,Governor10006520		188137472857428845	3	2020-09-15 03:48:56.703787	2020-09-15 03:48:56.703787
57222773636693151	Governor10006556,Governor10006522,Governor10006520		188137472857428845	3	2020-09-15 07:51:11.34498	2020-09-15 07:51:11.34498
3992953288946882355	Governor10006560,我比较特别,Governor10006556		2694133533970932102	4	2020-09-16 10:24:48.292485	2020-09-16 10:24:48.292485
6569429857201467107	old group		8999581573259430106	2	2020-08-07 11:17:10.005655	2020-08-07 11:17:10.005655
1847466864262776691	Governor10006210,Governor10006211,Governor10006212		8999581573259430106	3	2020-08-07 11:27:58.108797	2020-08-07 11:27:58.108797
7145037768768306206	Governor10006279,Governor10006298,Governor10006289		8544651895195766841	4	2020-08-31 10:05:57.19907	2020-08-31 10:05:57.19907
5114120711123843003	Governor10006279,Governor10006298,Governor10006289		8544651895195766841	4	2020-08-31 10:06:50.213775	2020-08-31 10:06:50.213775
6553152939034702434	Governor10006380,Governor10006381,Governor10006382		3913362401674305090	4	2020-09-04 03:27:20.751628	2020-09-04 03:27:20.751628
5738132283669128634	group2		6236962157120125250	3	2020-09-04 02:58:42.393764	2020-09-04 02:58:42.393764
6114809302637793709	group1		6030576862036077659	4	2020-09-03 11:37:49.414685	2020-09-03 11:37:49.414685
2702919077740007223	Governor10006380,Governor10006384,Governor10006381		6030576862036077659	4	2020-09-04 07:55:47.883901	2020-09-04 07:55:47.883901
\.


--
-- Data for Name: chat_groups_uids; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.chat_groups_uids (guid, gid, uuid, latest_timestamp, unread_count, modify_time, created_time) FROM stdin;
7161948643554999757	6569429857201467107	8999581573259430106	1596799030	0	2020-08-07 11:17:10.005655	2020-08-07 11:17:10.005655
7237032926188765470	6569429857201467107	6227828602660223846	1596799030	0	2020-08-07 11:17:10.005655	2020-08-07 11:17:10.005655
6995665112482465111	1847466864262776691	8999581573259430106	1596799678	0	2020-08-07 11:27:58.108797	2020-08-07 11:27:58.108797
5146483007111060303	1847466864262776691	6227828602660223846	1596799678	0	2020-08-07 11:27:58.108797	2020-08-07 11:27:58.108797
7403549157494163291	1847466864262776691	306813390447020112	1597031065	0	2020-08-10 03:44:25.885154	2020-08-10 03:44:25.885154
6939420644899240503	7145037768768306206	8544651895195766841	1598868357	0	2020-08-31 10:05:57.19907	2020-08-31 10:05:57.19907
2196611724010294909	7145037768768306206	1608327148726138071	1598868357	0	2020-08-31 10:05:57.19907	2020-08-31 10:05:57.19907
2659227951573882226	7145037768768306206	7447207625833516709	1598868357	0	2020-08-31 10:05:57.19907	2020-08-31 10:05:57.19907
7897781438534807721	7145037768768306206	3989321803933697049	1598868357	0	2020-08-31 10:05:57.19907	2020-08-31 10:05:57.19907
3035768475548744089	5114120711123843003	8544651895195766841	1598868410	0	2020-08-31 10:06:50.213775	2020-08-31 10:06:50.213775
5702761881735879675	5114120711123843003	1608327148726138071	1598868410	0	2020-08-31 10:06:50.213775	2020-08-31 10:06:50.213775
6532442516018236746	5114120711123843003	7447207625833516709	1598868410	0	2020-08-31 10:06:50.213775	2020-08-31 10:06:50.213775
7631940917044268202	5114120711123843003	3989321803933697049	1598868410	0	2020-08-31 10:06:50.213775	2020-08-31 10:06:50.213775
6831824394976155956	6114809302637793709	6236962157120125250	1599133069	0	2020-09-03 11:37:49.414685	2020-09-03 11:37:49.414685
4922799290447083478	6114809302637793709	42184729541320523	1599133069	0	2020-09-03 11:37:49.414685	2020-09-03 11:37:49.414685
7443039518231722193	5738132283669128634	3913362401674305090	1599188322	0	2020-09-04 02:58:42.393764	2020-09-04 02:58:42.393764
3638375477851181218	5738132283669128634	6236962157120125250	1599189498	0	2020-09-04 03:18:18.927517	2020-09-04 03:18:18.927517
237161990634042199	6553152939034702434	42184729541320523	1599190040	0	2020-09-04 03:27:20.751628	2020-09-04 03:27:20.751628
272177589129511496	6553152939034702434	6030576862036077659	1599190040	0	2020-09-04 03:27:20.751628	2020-09-04 03:27:20.751628
4610122409775274514	6553152939034702434	3913362401674305090	1599190059	0	2020-09-04 03:27:39.32223	2020-09-04 03:27:39.32223
8786827804424389008	6114809302637793709	6030576862036077659	1599199417	0	2020-09-04 06:03:37.475202	2020-09-04 06:03:37.475202
2393517831068780003	2702919077740007223	6236962157120125250	1599206147	0	2020-09-04 07:55:47.883901	2020-09-04 07:55:47.883901
6945887275648802933	2702919077740007223	42184729541320523	1599206147	0	2020-09-04 07:55:47.883901	2020-09-04 07:55:47.883901
2533568861621968180	2702919077740007223	6030576862036077659	1599206181	0	2020-09-04 07:56:21.762928	2020-09-04 07:56:21.762928
1971040140206208364	1389812494058547205	42184729541320523	1599207262	0	2020-09-04 08:14:22.574295	2020-09-04 08:14:22.574295
1727182458483747871	1389812494058547205	6030576862036077659	1599207262	0	2020-09-04 08:14:22.574295	2020-09-04 08:14:22.574295
8040526363730563847	1155267990789338481	5001370379009920352	1599550443	0	2020-09-08 07:34:03.271074	2020-09-08 07:34:03.271074
4684155700762674083	1155267990789338481	42184729541320523	1599550443	0	2020-09-08 07:34:03.271074	2020-09-08 07:34:03.271074
6831164302415877231	1155267990789338481	6984416433108191275	1599550443	0	2020-09-08 07:34:03.271074	2020-09-08 07:34:03.271074
2996666599634494182	1241873839936331673	3455115140489977330	1599788605	0	2020-09-11 01:43:25.830662	2020-09-09 01:49:22.372215
2357065939761860942	1241873839936331673	5271470258722089835	1599788605	0	2020-09-11 01:43:25.830662	2020-09-09 01:49:22.372215
7512446080572602937	1241873839936331673	8326186895181328012	1599788605	0	2020-09-11 01:43:25.830662	2020-09-09 01:49:22.372215
7201586851078046230	763106957186478718	188137472857428845	1600142416	1	2020-09-15 04:00:16.539286	2020-09-15 03:48:56.703787
8499296797499328169	763106957186478718	6627591757152218867	1600142416	1	2020-09-15 04:00:16.539286	2020-09-15 03:48:56.703787
2506915813703545821	763106957186478718	6309855246227066278	1600142416	1	2020-09-15 04:00:16.539286	2020-09-15 03:48:56.703787
3519496856095729948	57222773636693151	188137472857428845	0	0	2020-09-15 07:51:11.34498	2020-09-15 07:51:11.34498
6728316641307232050	3992953288946882355	2694133533970932102	1600322101	0	2020-09-17 05:55:01.777406	2020-09-16 10:24:48.292485
3091674412104079676	57222773636693151	6627591757152218867	0	2	2020-09-15 07:51:11.34498	2020-09-15 07:51:11.34498
6867139834823499754	57222773636693151	6309855246227066278	0	2	2020-09-15 07:51:11.34498	2020-09-15 07:51:11.34498
406763512436893707	3992953288946882355	3455115140489977330	1600322101	0	2020-09-17 05:55:01.777406	2020-09-16 10:24:48.292485
6106992634871145408	3992953288946882355	188137472857428845	1600322101	0	2020-09-17 05:55:01.777406	2020-09-16 10:24:48.292485
2267598931646966783	3992953288946882355	5271470258722089835	1600322101	0	2020-09-17 05:55:01.777406	2020-09-16 10:24:48.292485
\.


--
-- Data for Name: chat_messages; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.chat_messages (mid, send_id, to_id, content, created_timestamp, kind, modify_time, created_time) FROM stdin;
909947813164181766	3455115140489977330	5271470258722089835	hello	1599717721	3	2020-09-10 06:02:01.222452	2020-09-10 06:02:01.222452
5666166143472793131	5271470258722089835	3455115140489977330	test send message.	1599719484	3	2020-09-10 06:31:24.540332	2020-09-10 06:31:24.540332
5641113538051059997	5271470258722089835	3455115140489977330	test send p2p message.	1599722411	3	2020-09-10 07:20:11.820491	2020-09-10 07:20:11.820491
5401900774018539409	5271470258722089835	3455115140489977330	test send p2p message two.	1599722495	3	2020-09-10 07:21:35.357632	2020-09-10 07:21:35.357632
8549127038592706980	5271470258722089835	3455115140489977330	hi	1599725812	3	2020-09-10 08:16:52.103466	2020-09-10 08:16:52.103466
4975348337286396771	5271470258722089835	3455115140489977330	hi	1599725886	3	2020-09-10 08:18:06.127909	2020-09-10 08:18:06.127909
251043984718945813	5271470258722089835	3455115140489977330	hi	1599725931	3	2020-09-10 08:18:51.773502	2020-09-10 08:18:51.773502
9194737484201194378	6627591757152218867	329739458029569425	hi	1599730815	3	2020-09-10 09:40:15.467293	2020-09-10 09:40:15.467293
7838208370903171006	6627591757152218867	329739458029569425	hi 1	1599730818	3	2020-09-10 09:40:18.833276	2020-09-10 09:40:18.833276
8849249126449736714	8326186895181328012	329739458029569425	hi d	1599730885	3	2020-09-10 09:41:25.848453	2020-09-10 09:41:25.848453
8560088262366051364	5986398665897825204	5223911966781382373	1	1599731526	3	2020-09-10 09:52:06.416147	2020-09-10 09:52:06.416147
3812602012305786206	5986398665897825204	5223911966781382373	test send p2p message two.	1599731750	3	2020-09-10 09:55:50.634625	2020-09-10 09:55:50.634625
335398254819136920	8587893535813273098	6575428629280265870	hi 49	1599732349	3	2020-09-10 10:05:49.381742	2020-09-10 10:05:49.381742
267121149633130347	8587893535813273098	6575428629280265870	hi 49	1599732359	3	2020-09-10 10:05:59.043878	2020-09-10 10:05:59.043878
6980600576798913102	6575428629280265870	8587893535813273098	hi 49  how are you 	1599732520	3	2020-09-10 10:08:40.920911	2020-09-10 10:08:40.920911
5822168998204955333	6575428629280265870	8587893535813273098	hi 49  how are you 	1599732683	3	2020-09-10 10:11:23.733183	2020-09-10 10:11:23.733183
6741832571748512916	3455115140489977330	1241873839936331673	hi   team	1599734400	2	2020-09-10 10:40:00.557922	2020-09-10 10:40:00.557922
8236557009760801949	4959061375679117082	8147385450836295281	hi  6551	1599735133	3	2020-09-10 10:52:13.703445	2020-09-10 10:52:13.703445
7013952352269639250	3455115140489977330	5271470258722089835	1	1599736326	3	2020-09-10 11:12:06.632549	2020-09-10 11:12:06.632549
8265944706464794564	3455115140489977330	5271470258722089835	2	1599736328	3	2020-09-10 11:12:08.322041	2020-09-10 11:12:08.322041
8433889283950647003	3455115140489977330	5271470258722089835	3	1599736329	3	2020-09-10 11:12:09.679334	2020-09-10 11:12:09.679334
8802640344256926761	3455115140489977330	5271470258722089835	3	1599736332	3	2020-09-10 11:12:12.228191	2020-09-10 11:12:12.228191
1288703859577775676	3455115140489977330	1241873839936331673	1	1599736454	2	2020-09-10 11:14:14.489015	2020-09-10 11:14:14.489015
5668088332248028619	3455115140489977330	1241873839936331673	2	1599736456	2	2020-09-10 11:14:16.073494	2020-09-10 11:14:16.073494
8507104114532450774	3455115140489977330	1241873839936331673	3	1599736457	2	2020-09-10 11:14:17.25857	2020-09-10 11:14:17.25857
8548622793194291292	3455115140489977330	1241873839936331673	4	1599736458	2	2020-09-10 11:14:18.483113	2020-09-10 11:14:18.483113
6937144243451116849	3455115140489977330	1241873839936331673	234343534543	1599736462	2	2020-09-10 11:14:22.704987	2020-09-10 11:14:22.704987
282852171003951519	3455115140489977330	1241873839936331673	2rewdgfaegsdg4353523452435423523234524354352425	1599736466	2	2020-09-10 11:14:26.961712	2020-09-10 11:14:26.961712
7579495843666888583	1041955479368729000	1981557320191779142	hi  你好	1599789105	3	2020-09-11 01:51:45.087854	2020-09-11 01:51:45.087854
4012432721189100506	3455115140489977330	5271470258722089835	?	1599790490	3	2020-09-11 02:14:50.638903	2020-09-11 02:14:50.638903
2234274671120872953	3455115140489977330	8326186895181328012	asdf	1599790639	3	2020-09-11 02:17:19.385737	2020-09-11 02:17:19.385737
5683904993669198229	5986398665897825204	3455115140489977330	test send iashdfilasuhdfl message two.	1599816152963	3	2020-09-11 09:22:32.963213	2020-09-11 09:22:32.963213
7568660821616312431	5986398665897825204	3455115140489977330	test send sdsfwewe message two.	1599816170946	3	2020-09-11 09:22:50.946483	2020-09-11 09:22:50.946483
9219164017286470099	5986398665897825204	3455115140489977330	test send sdsfwewe message two.	1599816247893	3	2020-09-11 09:24:07.893059	2020-09-11 09:24:07.893059
451470734811013448	5986398665897825204	3455115140489977330	test send sdsfwewe message two.	1599816291872	3	2020-09-11 09:24:51.872191	2020-09-11 09:24:51.872191
7515380937484477004	5986398665897825204	3455115140489977330	test send oijoij message two.	1599816343823	3	2020-09-11 09:25:43.823085	2020-09-11 09:25:43.823085
2441561188030164526	5986398665897825204	3455115140489977330	test send iokkkkk message two.	1599816359643	3	2020-09-11 09:25:59.643209	2020-09-11 09:25:59.643209
1042253510265872467	5986398665897825204	3455115140489977330	test send iokkkkk message two.	1599816370032	3	2020-09-11 09:26:10.032042	2020-09-11 09:26:10.032042
1079528116055055753	5986398665897825204	3455115140489977330	test send iokkkkk message two.	1599816372372	3	2020-09-11 09:26:12.372782	2020-09-11 09:26:12.372782
9083655482131463895	5986398665897825204	3455115140489977330	test send iokkkkk message two.	1599816436512	3	2020-09-11 09:27:16.512559	2020-09-11 09:27:16.512559
4349667709771633740	5986398665897825204	3455115140489977330	test send ddddd message two.	1599823091971	3	2020-09-11 11:18:11.969961	2020-09-11 11:18:11.969961
2530401054308969775	5986398665897825204	3455115140489977330	test send ddddd message two.	1599823256229	3	2020-09-11 11:20:56.228231	2020-09-11 11:20:56.228231
6859314279516630929	5429881535936798637	6309855246227066278	?	1600081387321	3	2020-09-14 11:03:07.321531	2020-09-14 11:03:07.321531
297855973099765785	188137472857428845	6627591757152218867	?	1600082860738	3	2020-09-14 11:27:40.738466	2020-09-14 11:27:40.738466
4785636207653227890	188137472857428845	6627591757152218867	?	1600082878221	3	2020-09-14 11:27:58.220906	2020-09-14 11:27:58.220906
8519307503040265219	188137472857428845	6309855246227066278	hi 	1600135972241	3	2020-09-15 02:12:52.240756	2020-09-15 02:12:52.240756
6682522325557839593	188137472857428845	6309855246227066278	?	1600135986486	3	2020-09-15 02:13:06.485594	2020-09-15 02:13:06.485594
3590615306647659339	188137472857428845	6309855246227066278	?	1600135991108	3	2020-09-15 02:13:11.108051	2020-09-15 02:13:11.108051
6502951641774118751	188137472857428845	6627591757152218867	?	1600137288366	3	2020-09-15 02:34:48.366328	2020-09-15 02:34:48.366328
3340445672101466360	188137472857428845	6627591757152218867	hello	1600137291812	3	2020-09-15 02:34:51.812655	2020-09-15 02:34:51.812655
8313024847398875655	188137472857428845	6627591757152218867	are you ok ?	1600137297074	3	2020-09-15 02:34:57.074437	2020-09-15 02:34:57.074437
5178556871388098748	188137472857428845	6627591757152218867	are you are you are you are you are you 	1600137306983	3	2020-09-15 02:35:06.9837	2020-09-15 02:35:06.9837
91434489762112204	188137472857428845	6627591757152218867	are you are you are you are you are you are you are you are you are you are you are you 	1600137315174	3	2020-09-15 02:35:15.174686	2020-09-15 02:35:15.174686
8215926804092175000	188137472857428845	763106957186478718	hi  6520	1600141750778	2	2020-09-15 03:49:10.77797	2020-09-15 03:49:10.77797
2131991939707254856	188137472857428845	763106957186478718	讨论组发言	1600142030828	2	2020-09-15 03:53:50.82816	2020-09-15 03:53:50.82816
3390979940159569016	188137472857428845	763106957186478718	发言人  jq	1600142121939	2	2020-09-15 03:55:21.939358	2020-09-15 03:55:21.939358
3601543569449167767	188137472857428845	6309855246227066278	今天9.15	1600148029081	3	2020-09-15 05:33:49.08111	2020-09-15 05:33:49.08111
5436065531305390302	6309855246227066278	188137472857428845	我是 测试	1600148698086	3	2020-09-15 05:44:58.086371	2020-09-15 05:44:58.086371
4238459348840853492	188137472857428845	6309855246227066278	-》09150202	1600149789485	3	2020-09-15 06:03:09.485081	2020-09-15 06:03:09.485081
7233916263121888712	188137472857428845	763106957186478718	0202分组消息	1600149954458	2	2020-09-15 06:05:54.457836	2020-09-15 06:05:54.457836
8314327540621946899	3455115140489977330	5986398665897825204	test xianghu send.	1600150295594	3	2020-09-15 06:11:35.594421	2020-09-15 06:11:35.594421
3677301655678950511	5986398665897825204	3455115140489977330	test xianghu send two.	1600150613302	3	2020-09-15 06:16:53.3025	2020-09-15 06:16:53.3025
3567740049409296258	3455115140489977330	5986398665897825204	test sddffff send three.	1600150634555	3	2020-09-15 06:17:14.555484	2020-09-15 06:17:14.555484
1405988613177188842	188137472857428845	57222773636693151	hello  0915	1600156282823	2	2020-09-15 07:51:22.822927	2020-09-15 07:51:22.822927
8625598379390419980	188137472857428845	57222773636693151	 看清楚每个字符  真是的	1600158889682	2	2020-09-15 08:34:49.681707	2020-09-15 08:34:49.681707
7907030246010120798	6309855246227066278	188137472857428845	送手机	1600158955340	3	2020-09-15 08:35:55.339792	2020-09-15 08:35:55.339792
930448615888025897	188137472857428845	10006560	送手机	1600224304581	3	2020-09-16 02:45:04.580754	2020-09-16 02:45:04.580754
3099032236566764056	3455115140489977330	10006560	送手机	1600224315477	3	2020-09-16 02:45:15.476664	2020-09-16 02:45:15.476664
8509354583932613311	3455115140489977330	10006560	送手机	1600224360806	3	2020-09-16 02:46:00.806229	2020-09-16 02:46:00.806229
8178090499121369421	3455115140489977330	10006560	送手机	1600224362922	3	2020-09-16 02:46:02.922662	2020-09-16 02:46:02.922662
5723453921510895712	3455115140489977330	10006560	送手机	1600224363822	3	2020-09-16 02:46:03.822192	2020-09-16 02:46:03.822192
691914492224824054	2694133533970932102	3455115140489977330	love	1600224425597	3	2020-09-16 02:47:05.597638	2020-09-16 02:47:05.597638
3391460213538015232	2694133533970932102	3455115140489977330	love you	1600224769488	3	2020-09-16 02:52:49.488429	2020-09-16 02:52:49.488429
7981506936646072360	2694133533970932102	3455115140489977330	特别的哎	1600225949811	3	2020-09-16 03:12:29.810968	2020-09-16 03:12:29.810968
5250702246432465866	2694133533970932102	188137472857428845	hi 6556 	1600225969809	3	2020-09-16 03:12:49.809079	2020-09-16 03:12:49.809079
1914221868701327626	2694133533970932102	5271470258722089835	hi 6519	1600226013189	3	2020-09-16 03:13:33.189666	2020-09-16 03:13:33.189666
5985976595804034416	5271470258722089835	6829118970656486265	 我是第一次发言	1600226970423	1	2020-09-16 03:29:30.424424	2020-09-16 03:29:30.424424
3492850886300403249	5271470258722089835	6829118970656486265	 我是第二次发言	1600227017814	1	2020-09-16 03:30:17.814905	2020-09-16 03:30:17.814905
7348740715510603603	2694133533970932102	6829118970656486265	hello 6519	1600227697473	1	2020-09-16 03:41:37.473473	2020-09-16 03:41:37.473473
1937064844361685087	2694133533970932102	6829118970656486265	测试	1600245466184	1	2020-09-16 08:37:46.184462	2020-09-16 08:37:46.184462
2626611595469918035	2694133533970932102	6829118970656486265	测试下面	1600246165845	1	2020-09-16 08:49:25.84571	2020-09-16 08:49:25.84571
4375863467298681073	2694133533970932102	6829118970656486265	许博	1600246368233	1	2020-09-16 08:52:48.234092	2020-09-16 08:52:48.234092
6568865375643921337	2694133533970932102	6829118970656486265	小宝	1600246452408	1	2020-09-16 08:54:12.408506	2020-09-16 08:54:12.408506
1848566294581315614	2694133533970932102	6829118970656486265	zexin	1600246647544	1	2020-09-16 08:57:27.544738	2020-09-16 08:57:27.544738
4001474899948075077	2694133533970932102	6829118970656486265	time?	1600247988037	1	2020-09-16 09:19:48.038192	2020-09-16 09:19:48.038192
643125915858570549	2694133533970932102	6829118970656486265	jink	1600248178526	1	2020-09-16 09:22:58.526909	2020-09-16 09:22:58.526909
5638490032363508033	2694133533970932102	6829118970656486265	ok ?	1600248519701	1	2020-09-16 09:28:39.701904	2020-09-16 09:28:39.701904
169508549529012876	2694133533970932102	6829118970656486265	test?	1600249092119	1	2020-09-16 09:38:12.120293	2020-09-16 09:38:12.120293
5441506743603641625	2694133533970932102	6829118970656486265	test 1	1600249148337	1	2020-09-16 09:39:08.337303	2020-09-16 09:39:08.337303
6658657416162151145	2694133533970932102	6829118970656486265	test2	1600249252631	1	2020-09-16 09:40:52.631977	2020-09-16 09:40:52.631977
2479466075217799353	2694133533970932102	6829118970656486265	are you?	1600249471838	1	2020-09-16 09:44:31.839295	2020-09-16 09:44:31.839295
5366200974369431333	5271470258722089835	6829118970656486265	hello ?	1600250115895	1	2020-09-16 09:55:15.896368	2020-09-16 09:55:15.896368
1893708773671764804	5271470258722089835	6829118970656486265	进扩	1600250136375	1	2020-09-16 09:55:36.376072	2020-09-16 09:55:36.376072
7636954774313099894	6627591757152218867	6829118970656486265	shut up	1600250176048	1	2020-09-16 09:56:16.048543	2020-09-16 09:56:16.048543
7293988882540188260	2694133533970932102	6829118970656486265	?	1600250202713	1	2020-09-16 09:56:42.713712	2020-09-16 09:56:42.713712
5774781501709291785	6627591757152218867	6829118970656486265	why?	1600250217872	1	2020-09-16 09:56:57.872948	2020-09-16 09:56:57.872948
3896472028924789814	2694133533970932102	3455115140489977330	?	1600250413443	3	2020-09-16 10:00:13.442992	2020-09-16 10:00:13.442992
3507725288916840448	2694133533970932102	3455115140489977330	?	1600250421131	3	2020-09-16 10:00:21.131816	2020-09-16 10:00:21.131816
5346868567870953486	3455115140489977330	2694133533970932102	hello i am xubo	1600250696835	3	2020-09-16 10:04:56.835549	2020-09-16 10:04:56.835549
5036183069876126314	3455115140489977330	2694133533970932102	pb	1600250957070	3	2020-09-16 10:09:17.070023	2020-09-16 10:09:17.070023
1158799718283347617	3455115140489977330	2694133533970932102	dsm	1600251270111	3	2020-09-16 10:14:30.111377	2020-09-16 10:14:30.111377
7632095989240101887	3455115140489977330	2694133533970932102	?	1600251293230	3	2020-09-16 10:14:53.230298	2020-09-16 10:14:53.230298
2153045361480851729	3455115140489977330	2694133533970932102	什么情况	1600251310042	3	2020-09-16 10:15:10.04266	2020-09-16 10:15:10.04266
8741132985631550163	3455115140489977330	2694133533970932102	对面	1600251336075	3	2020-09-16 10:15:36.075812	2020-09-16 10:15:36.075812
67760874720893008	2694133533970932102	3992953288946882355	?	1600251898156	2	2020-09-16 10:24:58.156126	2020-09-16 10:24:58.156126
1893764839801197640	2694133533970932102	3992953288946882355	？	1600252226253	2	2020-09-16 10:30:26.25294	2020-09-16 10:30:26.25294
7197446908177403764	2694133533970932102	3992953288946882355	hello	1600254178245	2	2020-09-16 11:02:58.245445	2020-09-16 11:02:58.245445
2596729432065433154	3455115140489977330	3992953288946882355	in group	1600254211431	2	2020-09-16 11:03:31.431438	2020-09-16 11:03:31.431438
511114777548493679	3455115140489977330	3992953288946882355	are you ok ?	1600254232739	2	2020-09-16 11:03:52.738916	2020-09-16 11:03:52.738916
1493315725265703558	6627591757152218867	4298352602374805827	are you ok ?	1600254449734	3	2020-09-16 11:07:29.734296	2020-09-16 11:07:29.734296
1651490685396199939	6627591757152218867	2694133533970932102	are you ok ?	1600254748900	3	2020-09-16 11:12:28.90034	2020-09-16 11:12:28.90034
3910047818218066147	2694133533970932102	6627591757152218867	?	1600254766995	3	2020-09-16 11:12:46.995692	2020-09-16 11:12:46.995692
1685222156031112655	2694133533970932102	6627591757152218867	you are	1600254792563	3	2020-09-16 11:13:12.563599	2020-09-16 11:13:12.563599
3950289546535623401	2694133533970932102	6627591757152218867	hehe	1600254908981	3	2020-09-16 11:15:08.981433	2020-09-16 11:15:08.981433
6732596032842965496	2694133533970932102	3455115140489977330	hello	1600311176871	3	2020-09-17 02:52:56.871529	2020-09-17 02:52:56.871529
2903558162678744330	2694133533970932102	188137472857428845	nani?	1600311189016	3	2020-09-17 02:53:09.015862	2020-09-17 02:53:09.015862
660297180316770163	2694133533970932102	5271470258722089835	6519	1600321863108	3	2020-09-17 05:51:03.10798	2020-09-17 05:51:03.10798
3713109356945321260	2694133533970932102	3992953288946882355	in group	1600322044445	2	2020-09-17 05:54:04.445008	2020-09-17 05:54:04.445008
3476714523385395786	6627591757152218867	6829118970656486265	are you ok ?	1600322218800	1	2020-09-17 05:56:58.800776	2020-09-17 05:56:58.800776
8937897745402545750	2694133533970932102	3455115140489977330	old	1600323487664	3	2020-09-17 06:18:07.664559	2020-09-17 06:18:07.664559
6944470527847145428	2694133533970932102	5271470258722089835	second	1600323499673	3	2020-09-17 06:18:19.673747	2020-09-17 06:18:19.673747
293878084523839627	2694133533970932102	188137472857428845	thread	1600323821493	3	2020-09-17 06:23:41.493067	2020-09-17 06:23:41.493067
2672606328034966908	4027022891871908826	6829118970656486265	hi	1600402491994	1	2020-09-18 04:14:51.995152	2020-09-18 04:14:51.995152
611209788507649081	5271470258722089835	4027022891871908826	hi 7	1600402632796	3	2020-09-18 04:17:12.796558	2020-09-18 04:17:12.796558
8935792316151027467	5271470258722089835	4027022891871908826	hi 78	1600402663038	3	2020-09-18 04:17:43.038575	2020-09-18 04:17:43.038575
\.


--
-- Data for Name: enemys; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.enemys (eid, enemy_name, model_path, thumbnail, max_hp, attack_power, move_speed, max_mana, defense, animation_hit_delay, spawn_style_class, bp_enemy, ap_enemy, skm_enemy, aenemy_die, modify_time, created_time) FROM stdin;
6905828721094891599	Bat	/Game/Match3RPG/Blueprints/Units/Enemy/MBat	Texture2D'/Game/Commons/Textures/Enemy/Bat.Bat'	30	0	600	0	5	0.600000024	/Script/TheForeAwakensCPlus.MRunSpawn	Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MBat.MBat_C'	AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/Bat/ABP_Bat.ABP_Bat_C'	SkeletalMesh'/Game/RPGMonsterWavePBR/Meshes/Bat/Bat_SK.Bat_SK'	AnimSequence'/Game/RPGMonsterWavePBR/Animations/Bat/Die_Bat_Anim.Die_Bat_Anim'	2020-09-08 08:10:15.817241	2020-09-08 08:10:15.817241
6946808681194466747	Dragon	/Game/Match3RPG/Blueprints/Units/Enemy/MDragon	Texture2D'/Game/Commons/Textures/Enemy/Dragon.Dragon'	53	0	600	0	5	0.600000024	/Script/TheForeAwakensCPlus.MRunSpawn	Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MDragon.MDragon_C'	AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/Dragon/ABP_Dragon.ABP_Dragon_C'	SkeletalMesh'/Game/RPGMonsterWavePBR/Meshes/Dragon/Dragon_SK.Dragon_SK'	AnimSequence'/Game/RPGMonsterWavePBR/Animations/Dragon/Die_Dragon_Anim.Die_Dragon_Anim'	2020-09-08 08:10:15.817241	2020-09-08 08:10:15.817241
7775215474871160888	Evilmage	/Game/Match3RPG/Blueprints/Units/Enemy/MEvilmage	Texture2D'/Game/Commons/Textures/Enemy/Evilmage.Evilmage'	56	0	600	0	5	0.5	/Script/TheForeAwakensCPlus.MRunSpawn	Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MEvilmage.MEvilmage_C'	AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/EvilMage/OneMeshCharacter/ABP_EvilMage_OneMesh.ABP_EvilMage_OneMesh_C'	SkeletalMesh'/Game/RPGMonsterWavePBR/Meshes/EvilMage/OneMeshCharacter/EvilMage_OneMesh_SK.EvilMage_OneMesh_SK'	AnimSequence'/Game/RPGMonsterWavePBR/Animations/EvilMage/Die_EvilMage_Anim.Die_EvilMage_Anim'	2020-09-08 08:10:15.817241	2020-09-08 08:10:15.817241
338676047550973005	Ghoul	/Game/Match3RPG/Blueprints/Units/Enemy/MGhoul	Texture2D'/Game/Commons/Textures/Enemy/Ghoul.Ghoul'	30	0	600	0	6	0.5	/Script/TheForeAwakensCPlus.MRunSpawn	Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MGhoul.MGhoul_C'	AnimBlueprint'/Game/Ghoul_Set/Ghoul/ABP_Ghoul.ABP_Ghoul_C'	SkeletalMesh'/Game/Ghoul_Set/Ghoul/SK_Ghoul.SK_Ghoul'	AnimSequence'/Game/Ghoul_Set/Ghoul/Anims/Ghoul_Die.Ghoul_Die'	2020-09-08 08:10:15.817241	2020-09-08 08:10:15.817241
8078812079154036494	Ghoul_Boss	/Game/Match3RPG/Blueprints/Units/Enemy/MGhoul_Boss	Texture2D'/Game/Commons/Textures/Enemy/Ghoul_Boss.Ghoul_Boss	100	0	600	0	8	0.5	/Script/TheForeAwakensCPlus.MRunSpawn	Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MGhoul_Boss.MGhoul_Boss_C'	AnimBlueprint'/Game/Ghoul_Set/Ghoul_Boss/ABP_Ghoul_Boss.ABP_Ghoul_Boss_C'	SkeletalMesh'/Game/Ghoul_Set/Ghoul_Boss/SK_Ghoul_Boss.SK_Ghoul_Boss'	AnimSequence'/Game/Ghoul_Set/Ghoul_Boss/Anims/Ghoul_Boss_Die.Ghoul_Boss_Die'	2020-09-08 08:10:15.817241	2020-09-08 08:10:15.817241
5434195190315914010	Ghoul_Festering	/Game/Match3RPG/Blueprints/Units/Enemy/MGhoul_Festering	Texture2D'/Game/Commons/Textures/Enemy/Ghoul_Festering.Ghoul_Festering'	100	0	600	0	7	0.5	/Script/TheForeAwakensCPlus.MRunSpawn	Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MGhoul_Festering.MGhoul_Festering_C'	AnimBlueprint'/Game/Ghoul_Set/Ghoul_Festering/ABP_Ghoul_festering.ABP_Ghoul_Festering_C'	SkeletalMesh'/Game/Ghoul_Set/Ghoul_Festering/SK_Ghoul_Festering.SK_Ghoul_Festering'	AnimSequence'/Game/Ghoul_Set/Ghoul_Festering/Anims/Festering_Ghoul_Die.Festering_Ghoul_Die'	2020-09-08 08:10:15.817241	2020-09-08 08:10:15.817241
5683408629523021734	Ghoul_Scavenger	/Game/Match3RPG/Blueprints/Units/Enemy/MGhoul_Scavenger	Texture2D'/Game/Commons/Textures/Enemy/Ghoul_Scavenger.Ghoul_Scavenger'	78	0	600	0	5	0.5	/Script/TheForeAwakensCPlus.MRunSpawn	Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MGhoul_Scavenger.MGhoul_Scavenger_C'	AnimBlueprint'/Game/Ghoul_Set/Ghoul_Scavenger/ABP_Ghoul_Scavenger.ABP_Ghoul_Scavenger_C'	SkeletalMesh'/Game/Ghoul_Set/Ghoul_Scavenger/SK_Ghoul_Scavenger.SK_Ghoul_Scavenger'	AnimSequence'/Game/Ghoul_Set/Ghoul_Scavenger/Anims/Ghoul_Scavenger_Die.Ghoul_Scavenger_Die'	2020-09-08 08:10:15.817241	2020-09-08 08:10:15.817241
2647844488543154810	Golem	/Game/Match3RPG/Blueprints/Units/Enemy/MGolem	Texture2D'/Game/Commons/Textures/Enemy/Golem.Golem'	50	0	600	0	6	0.5	/Script/TheForeAwakensCPlus.MRunSpawn	Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MGolem.MGolem_C'	AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/Golem/ABP_Golem.ABP_Golem_C'	SkeletalMesh'/Game/RPGMonsterWavePBR/Meshes/Golem/Golem_SK.Golem_SK'	AnimSequence'/Game/RPGMonsterWavePBR/Animations/Golem/Die_Golem_Anim.Die_Golem_Anim'	2020-09-08 08:10:15.817241	2020-09-08 08:10:15.817241
3779333134542240560	MonsterPlant	/Game/Match3RPG/Blueprints/Units/Enemy/MMonsterPlant	Texture2D'/Game/Commons/Textures/Enemy/MonsterPlant.MonsterPlant'	30	0	600	0	5	0.5	/Script/TheForeAwakensCPlus.MRunSpawn	Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MMonsterPlant.MMonsterPlant_C'	AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/MonsterPlant/ABS_MonsterPlant.ABS_MonsterPlant_C'	SkeletalMesh'/Game/RPGMonsterWavePBR/Meshes/MonsterPlant/MonsterPlant_SK.MonsterPlant_SK'	AnimSequence'/Game/RPGMonsterWavePBR/Animations/MonsterPlant/Die_MP_Anim.Die_MP_Anim'	2020-09-08 08:10:15.817241	2020-09-08 08:10:15.817241
9088913670112212976	Orc	/Game/Match3RPG/Blueprints/Units/Enemy/MOrc	Texture2D'/Game/Commons/Textures/Enemy/Orc.Orc'	60	0	600	0	6	0.5	/Script/TheForeAwakensCPlus.MRunSpawn	Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MOrc.MOrc_C'	AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/Orc/OneMeshCharacter/ABP_Orc_OneMesh.ABP_Orc_OneMesh_C'	SkeletalMesh'/Game/RPGMonsterWavePBR/Meshes/Orc/OneMeshCharacter/Orc_OneMesh_SK.Orc_OneMesh_SK'	AnimSequence'/Game/RPGMonsterWavePBR/Animations/Orc/Die_Orc_Anim.Die_Orc_Anim'	2020-09-08 08:10:15.817241	2020-09-08 08:10:15.817241
7340739757772068518	Skeleton	/Game/Match3RPG/Blueprints/Units/Enemy/MSkeleton	Texture2D'/Game/Commons/Textures/Enemy/Bat.Bat'	60	0	600	0	6	0.5	/Script/TheForeAwakensCPlus.MRunSpawn	Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MSkeleton.MSkeleton_C'	AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/Skeleton/OneMeshCharacter/ABP_Skeleton_OneMesh.ABP_Skeleton_OneMesh_C'	SkeletalMesh'/Game/RPGMonsterWavePBR/Meshes/Skeleton/OneMeshCharacter/Skeleton_OneMesh_SK.Skeleton_OneMesh_SK'	AnimSequence'/Game/RPGMonsterWavePBR/Animations/Skeleton/Die_Skeleton_Anim.Die_Skeleton_Anim'	2020-09-08 08:10:15.817241	2020-09-08 08:10:15.817241
75218728476491326	Slime	/Game/Match3RPG/Blueprints/Units/Enemy/MSlime	Texture2D'/Game/Commons/Textures/Enemy/Slime.Slime'	36	0	600	0	6	0.5	/Script/TheForeAwakensCPlus.MRunSpawn	Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MSlime.MSlime_C'	AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/Slime/ABP_Slime.ABP_Slime_C'	SkeletalMesh'/Game/RPGMonsterWavePBR/Meshes/Slime/Slime_SK.Slime_SK'	AnimSequence'/Game/RPGMonsterWavePBR/Animations/Slime/Die_Slime_Anim.Die_Slime_Anim'	2020-09-08 08:10:15.817241	2020-09-08 08:10:15.817241
4278179493359130662	Spider	/Game/Match3RPG/Blueprints/Units/Enemy/MSpider	Texture2D'/Game/Commons/Textures/Enemy/Spider.Spider'	40	0	600	0	5	0.5	/Script/TheForeAwakensCPlus.MRunSpawn	Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MSpider.MSpider_C'	AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/Spider/ABP_Spider.ABP_Spider_C'	SkeletalMesh'/Game/RPGMonsterWavePBR/Meshes/Spider/Spider_SK.Spider_SK'	AnimSequence'/Game/RPGMonsterWavePBR/Animations/Spider/Die_Spider_Anim.Die_Spider_Anim'	2020-09-08 08:10:15.817241	2020-09-08 08:10:15.817241
922787162695561417	TurleShell	/Game/Match3RPG/Blueprints/Units/Enemy/MTurleShell	Texture2D'/Game/Commons/Textures/Enemy/TurleShell.TurleShell'	20	0	600	0	5	0.5	/Script/TheForeAwakensCPlus.MRunSpawn	Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MTurleShell.MTurleShell_C'	AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/TurtleShell/ABP_TurtleShell.ABP_TurtleShell_C'	SkeletalMesh'/Game/RPGMonsterWavePBR/Meshes/TurtleShell/TurtleShell_SK.TurtleShell_SK'	AnimSequence'/Game/RPGMonsterWavePBR/Animations/TurtleShell/Die_TurtleShell_Anim.Die_TurtleShell_Anim'	2020-09-08 08:10:15.817241	2020-09-08 08:10:15.817241
\.


--
-- Data for Name: equipment_kinds; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.equipment_kinds (kid, name, kind, modify_time, created_time) FROM stdin;
407971465484766409	Weapon	1	2020-09-08 08:10:15.821733	2020-09-08 08:10:15.821733
70987388267557506	Shield	2	2020-09-08 08:10:15.821733	2020-09-08 08:10:15.821733
2883628719373703242	Armor	3	2020-09-08 08:10:15.821733	2020-09-08 08:10:15.821733
\.


--
-- Data for Name: equipments; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.equipments (eid, kid, name, thumbnail, price, hp, multiplier, kind, is_default, modify_time, created_time) FROM stdin;
6750612841634615393	2883628719373703242	First Armor	Texture2D'/Game/Commons/Textures/Equipment/Clothes/Equipment_Clothes_1.Equipment_Clothes_1'	1	5	1	3	2	2020-09-08 08:10:15.823206	2020-09-08 08:10:15.823206
6505506293171167408	2883628719373703242	Second Armor	Texture2D'/Game/Commons/Textures/Equipment/Clothes/Equipment_Clothes_2.Equipment_Clothes_2'	240	10	1	3	1	2020-09-08 08:10:15.823206	2020-09-08 08:10:15.823206
5488916325371064141	2883628719373703242	Third Armor	Texture2D'/Game/Commons/Textures/Equipment/Clothes/Equipment_Clothes_3.Equipment_Clothes_3'	20	20	1	3	1	2020-09-08 08:10:15.823206	2020-09-08 08:10:15.823206
3984697827620770275	70987388267557506	First Shield	Texture2D'/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_1.Equipment_Shield_1'	15	0	1	2	2	2020-09-08 08:10:15.823206	2020-09-08 08:10:15.823206
9075465603310523524	70987388267557506	Secend Shield	Texture2D'/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_2.Equipment_Shield_2'	115	0	1	2	1	2020-09-08 08:10:15.823206	2020-09-08 08:10:15.823206
1862664629529357697	70987388267557506	Third Shield	Texture2D'/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_3.Equipment_Shield_3'	215	0	2	2	1	2020-09-08 08:10:15.823206	2020-09-08 08:10:15.823206
8791584697569536544	407971465484766409	First Weapon	Texture2D'/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_1.Equipment_Weapon_1'	1	0	1	1	2	2020-09-08 08:10:15.823206	2020-09-08 08:10:15.823206
3667706950380863445	407971465484766409	Second Weapon	Texture2D'/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_2.Equipment_Weapon_2'	240	0	2	1	1	2020-09-08 08:10:15.823206	2020-09-08 08:10:15.823206
3534513442598030609	407971465484766409	Third Weapon	Texture2D'/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_3.Equipment_Weapon_3'	900	0	3	1	1	2020-09-08 08:10:15.823206	2020-09-08 08:10:15.823206
8349886201892374001	407971465484766409	 Angel Staff	Texture2D'/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_1.Equipment_Weapon_1'	0	0	0.0500000007	1	1	2020-09-17 07:33:50.520082	2020-09-17 07:33:50.520082
1982394797412105466	70987388267557506	French Robe	Texture2D'/Game/Commons/Textures/Equipment/Clothes/Equipment_Clothes_1.Equipment_Clothes_1'	0	5	0	2	1	2020-09-17 07:33:50.520082	2020-09-17 07:33:50.520082
6615567943590654816	2883628719373703242	Angel Shield	Texture2D'/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_1.Equipment_Shield_1'	0	10	0	3	1	2020-09-17 07:33:50.520082	2020-09-17 07:33:50.520082
5515803116356807042	407971465484766409	 Angel Staff	Texture2D'/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_1.Equipment_Weapon_1'	0	0	0.0500000007	1	1	2020-09-17 08:42:08.25873	2020-09-17 08:42:08.25873
4789151618626812118	70987388267557506	French Robe	Texture2D'/Game/Commons/Textures/Equipment/Clothes/Equipment_Clothes_1.Equipment_Clothes_1'	0	5	0	2	1	2020-09-17 08:42:08.25873	2020-09-17 08:42:08.25873
8712897837927593025	2883628719373703242	Angel Shield	Texture2D'/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_1.Equipment_Shield_1'	0	10	0	3	1	2020-09-17 08:42:08.25873	2020-09-17 08:42:08.25873
6678320473774019377	407971465484766409	 Angel Staff	Texture2D'/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_1.Equipment_Weapon_1'	0	0	0.0500000007	1	1	2020-09-17 10:52:28.248436	2020-09-17 10:52:28.248436
800339332447284818	70987388267557506	French Robe	Texture2D'/Game/Commons/Textures/Equipment/Clothes/Equipment_Clothes_1.Equipment_Clothes_1'	0	5	0	2	1	2020-09-17 10:52:28.248436	2020-09-17 10:52:28.248436
7775730202917710895	2883628719373703242	Angel Shield	Texture2D'/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_1.Equipment_Shield_1'	0	10	0	3	1	2020-09-17 10:52:28.248436	2020-09-17 10:52:28.248436
7367515832694383145	407971465484766409	 Angel Staff	Texture2D'/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_1.Equipment_Weapon_1'	0	0	0.0500000007	1	1	2020-09-15 08:28:36.069954	2020-09-15 08:28:36.069954
4399569311271143941	70987388267557506	French Robe	Texture2D'/Game/Commons/Textures/Equipment/Clothes/Equipment_Clothes_1.Equipment_Clothes_1'	0	5	0	2	1	2020-09-15 08:28:36.069954	2020-09-15 08:28:36.069954
2230968894271688632	2883628719373703242	Angel Shield	Texture2D'/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_1.Equipment_Shield_1'	0	10	0	3	1	2020-09-15 08:28:36.069954	2020-09-15 08:28:36.069954
549236052085417001	407971465484766409	 Angel Staff	Texture2D'/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_1.Equipment_Weapon_1'	0	0	0.0500000007	1	1	2020-09-17 08:34:36.690026	2020-09-17 08:34:36.690026
8767010983479355701	70987388267557506	French Robe	Texture2D'/Game/Commons/Textures/Equipment/Clothes/Equipment_Clothes_1.Equipment_Clothes_1'	0	5	0	2	1	2020-09-17 08:34:36.690026	2020-09-17 08:34:36.690026
6530684588090682561	2883628719373703242	Angel Shield	Texture2D'/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_1.Equipment_Shield_1'	0	10	0	3	1	2020-09-17 08:34:36.690026	2020-09-17 08:34:36.690026
7705326282310627813	407971465484766409	 Angel Staff	Texture2D'/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_1.Equipment_Weapon_1'	0	0	0.0500000007	1	1	2020-09-17 10:38:50.806261	2020-09-17 10:38:50.806261
4064786836449883542	70987388267557506	French Robe	Texture2D'/Game/Commons/Textures/Equipment/Clothes/Equipment_Clothes_1.Equipment_Clothes_1'	0	5	0	2	1	2020-09-17 10:38:50.806261	2020-09-17 10:38:50.806261
544238997007077338	2883628719373703242	Angel Shield	Texture2D'/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_1.Equipment_Shield_1'	0	10	0	3	1	2020-09-17 10:38:50.806261	2020-09-17 10:38:50.806261
1317622610439774139	407971465484766409	 Angel Staff	Texture2D'/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_1.Equipment_Weapon_1'	0	0	0.0500000007	1	1	2020-09-17 11:20:00.840609	2020-09-17 11:20:00.840609
7411409667987130823	70987388267557506	French Robe	Texture2D'/Game/Commons/Textures/Equipment/Clothes/Equipment_Clothes_1.Equipment_Clothes_1'	0	5	0	2	1	2020-09-17 11:20:00.840609	2020-09-17 11:20:00.840609
6199336508882440580	2883628719373703242	Angel Shield	Texture2D'/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_1.Equipment_Shield_1'	0	10	0	3	1	2020-09-17 11:20:00.840609	2020-09-17 11:20:00.840609
\.


--
-- Data for Name: friends; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.friends (fid, uuid_a, uuid_b, state, modify_time, created_time) FROM stdin;
8734924292263472971	1822034910362921280	7741055111594326769	1	2020-08-06 03:45:21.887556	2020-08-06 03:45:21.887556
1694288034131571173	1822034910362921280	7741055111594326769	1	2020-08-06 03:58:05.52437	2020-08-06 03:58:05.52437
3688297419367866477	1822034910362921280	7741055111594326769	1	2020-08-06 03:58:15.924615	2020-08-06 03:58:15.924615
3179756356165422574	1822034910362921280	1719434652837090626	1	2020-08-06 06:30:14.207402	2020-08-06 06:30:14.207402
6775426330774050199	1822034910362921280	619949171856467096	2	2020-08-06 06:29:12.047041	2020-08-06 06:29:12.047041
3630661065621282141	619949171856467096	1822034910362921280	2	2020-08-06 06:52:41.613071	2020-08-06 06:52:41.613071
6447059014077691332	1719434652837090626	1822034910362921280	1	2020-08-06 06:57:35.499297	2020-08-06 06:57:35.499297
8606055970085298076	1273575576218516600	2451557597158638279	2	2020-08-06 07:06:05.938058	2020-08-06 07:06:05.938058
3228636034127099678	2451557597158638279	1273575576218516600	2	2020-08-06 07:06:30.019876	2020-08-06 07:06:30.019876
5623819239504923402	6236962157120125250	5001370379009920352	2	2020-09-03 11:30:40.720905	2020-09-03 11:30:40.720905
7175598285934404888	5001370379009920352	6236962157120125250	2	2020-09-03 11:31:34.766635	2020-09-03 11:31:34.766635
4370532022475328391	1273575576218516600	2562300244197550099	2	2020-08-06 08:11:50.216604	2020-08-06 08:11:50.216604
6643500283198198089	2562300244197550099	1273575576218516600	2	2020-08-06 08:12:05.063669	2020-08-06 08:12:05.063669
619777751334119964	42184729541320523	5001370379009920352	2	2020-09-03 11:31:00.375256	2020-09-03 11:31:00.375256
8697259561365611231	5001370379009920352	42184729541320523	2	2020-09-03 11:37:32.862024	2020-09-03 11:37:32.862024
3465486051884453559	6030576862036077659	5001370379009920352	2	2020-09-03 11:31:05.322556	2020-09-03 11:31:05.322556
1848508789591576892	793426562737268076	1273575576218516600	2	2020-08-06 09:24:33.67998	2020-08-06 09:24:33.67998
3813179051040561774	1273575576218516600	793426562737268076	2	2020-08-06 09:25:55.005597	2020-08-06 09:25:55.005597
926848421124778889	5001370379009920352	6030576862036077659	2	2020-09-03 11:37:34.742011	2020-09-03 11:37:34.742011
7616043970991969073	3913362401674305090	5001370379009920352	2	2020-09-03 11:31:09.45593	2020-09-03 11:31:09.45593
1709373779422294676	7990476522529993677	7990476522529993677	1	2020-08-07 08:41:39.79026	2020-08-07 08:41:39.79026
3244122920817224839	7990476522529993677	1273575576218516600	2	2020-08-07 08:42:18.89191	2020-08-07 08:42:18.89191
2638875344670235752	1273575576218516600	7990476522529993677	2	2020-08-07 08:42:53.374255	2020-08-07 08:42:53.374255
8233525027011499097	8999581573259430106	306813390447020112	2	2020-08-07 09:40:24.456089	2020-08-07 09:40:24.456089
6243136492530125643	306813390447020112	8999581573259430106	2	2020-08-07 09:41:15.370511	2020-08-07 09:41:15.370511
6182665576401920626	8999581573259430106	3895258190548337274	1	2020-08-07 09:42:01.115999	2020-08-07 09:42:01.115999
4207662558726254585	8999581573259430106	6227828602660223846	2	2020-08-07 09:41:53.049488	2020-08-07 09:41:53.049488
7871393286673814366	6227828602660223846	8999581573259430106	2	2020-08-07 09:42:25.650761	2020-08-07 09:42:25.650761
5889582799835323375	8999581573259430106	3824269885763815063	2	2020-08-07 09:41:57.397164	2020-08-07 09:41:57.397164
5440301895965149748	3824269885763815063	8999581573259430106	2	2020-08-07 09:43:07.168826	2020-08-07 09:43:07.168826
5688697469177503047	4130760339889208668	4130760339889208668	1	2020-08-26 08:53:06.391841	2020-08-26 08:53:06.391841
6107842178284306513	5001370379009920352	3913362401674305090	2	2020-09-03 11:37:36.724934	2020-09-03 11:37:36.724934
6972907595665498038	42184729541320523	6984416433108191275	1	2020-09-04 10:26:43.805713	2020-09-04 10:26:43.805713
5148356811406727161	6030576862036077659	6984416433108191275	1	2020-09-04 10:26:47.975608	2020-09-04 10:26:47.975608
7975075394116798228	3935501584199374491	3199890358306707442	1	2020-09-02 01:42:26.330818	2020-09-02 01:42:26.330818
3647920490188200114	3989321803933697049	3199890358306707442	2	2020-09-02 01:42:03.855979	2020-09-02 01:42:03.855979
8852554598750740195	3199890358306707442	3989321803933697049	2	2020-09-02 01:43:27.314082	2020-09-02 01:43:27.314082
8915903995574872652	1608327148726138071	3199890358306707442	2	2020-09-02 01:42:14.942828	2020-09-02 01:42:14.942828
1850687058617490425	3199890358306707442	1608327148726138071	2	2020-09-02 01:43:29.745823	2020-09-02 01:43:29.745823
4389934618465530080	7447207625833516709	3199890358306707442	2	2020-09-02 01:42:10.968586	2020-09-02 01:42:10.968586
8854531693203322730	3199890358306707442	7447207625833516709	2	2020-09-02 01:43:51.574782	2020-09-02 01:43:51.574782
6163204960019604889	1549277623127607740	3199890358306707442	2	2020-09-02 01:42:20.293692	2020-09-02 01:42:20.293692
3886783465669280742	3199890358306707442	1549277623127607740	2	2020-09-02 01:43:53.62428	2020-09-02 01:43:53.62428
2777820082829091123	3935501584199374491	7819648131060985212	2	2020-09-02 02:17:29.427892	2020-09-02 02:17:29.427892
6458142684506398393	7819648131060985212	3935501584199374491	2	2020-09-02 02:17:40.546917	2020-09-02 02:17:40.546917
8464574614086178736	6236962157120125250	6984416433108191275	1	2020-09-04 10:26:55.084272	2020-09-04 10:26:55.084272
5420840026925567615	5001370379009920352	6984416433108191275	2	2020-09-04 10:26:38.867319	2020-09-04 10:26:38.867319
7058799623102797570	6984416433108191275	5001370379009920352	2	2020-09-04 10:28:01.849554	2020-09-04 10:28:01.849554
4726260755163821292	3913362401674305090	6984416433108191275	2	2020-09-04 10:26:51.870741	2020-09-04 10:26:51.870741
3719924846702306635	6984416433108191275	3913362401674305090	2	2020-09-04 10:37:36.518224	2020-09-04 10:37:36.518224
815436823589950009	6309855246227066278	3455115140489977330	1	2020-09-08 09:11:31.665562	2020-09-08 09:11:31.665562
162251608608029407	6627591757152218867	3455115140489977330	1	2020-09-08 09:11:38.494773	2020-09-08 09:11:38.494773
671195655485076496	329739458029569425	3455115140489977330	1	2020-09-08 09:11:41.93754	2020-09-08 09:11:41.93754
1901404956128584519	5271470258722089835	3455115140489977330	2	2020-09-08 09:11:26.720723	2020-09-08 09:11:26.720723
3677516901121969828	3455115140489977330	5271470258722089835	2	2020-09-08 10:00:44.007393	2020-09-08 10:00:44.007393
952804018509257764	8326186895181328012	3455115140489977330	2	2020-09-08 09:11:34.734045	2020-09-08 09:11:34.734045
1612308016637759974	3455115140489977330	8326186895181328012	2	2020-09-09 01:49:13.020699	2020-09-09 01:49:13.020699
5666927503922585825	5271470258722089835	5429881535936798637	1	2020-09-14 11:02:31.101383	2020-09-14 11:02:31.101383
680057009063672514	8326186895181328012	5429881535936798637	1	2020-09-14 11:02:39.310419	2020-09-14 11:02:39.310419
326427988623254355	329739458029569425	5429881535936798637	1	2020-09-14 11:02:42.701311	2020-09-14 11:02:42.701311
2782882779368157969	6627591757152218867	5429881535936798637	1	2020-09-14 11:02:45.957998	2020-09-14 11:02:45.957998
8576491828055447401	6309855246227066278	5429881535936798637	2	2020-09-14 11:02:35.549814	2020-09-14 11:02:35.549814
4665804678843846234	5429881535936798637	6309855246227066278	2	2020-09-14 11:03:13.162317	2020-09-14 11:03:13.162317
7299334702434732276	6627591757152218867	188137472857428845	2	2020-09-14 11:27:30.141476	2020-09-14 11:27:30.141476
4996263763840861806	188137472857428845	6627591757152218867	2	2020-09-14 11:27:35.822172	2020-09-14 11:27:35.822172
2898994769222127771	6309855246227066278	188137472857428845	2	2020-09-14 11:29:03.530296	2020-09-14 11:29:03.530296
1300740563972217845	188137472857428845	6309855246227066278	2	2020-09-14 11:29:11.044129	2020-09-14 11:29:11.044129
987992730449718350	3455115140489977330	2694133533970932102	2	2020-09-16 02:46:24.503161	2020-09-16 02:46:24.503161
2787635377945121503	2694133533970932102	3455115140489977330	2	2020-09-16 02:46:38.370477	2020-09-16 02:46:38.370477
4304355457154938373	188137472857428845	2694133533970932102	2	2020-09-16 02:46:28.131536	2020-09-16 02:46:28.131536
3283259725227240239	2694133533970932102	188137472857428845	2	2020-09-16 02:46:42.864203	2020-09-16 02:46:42.864203
2739585753331463744	5271470258722089835	2694133533970932102	2	2020-09-16 03:13:08.313658	2020-09-16 03:13:08.313658
8271111193317569989	2694133533970932102	5271470258722089835	2	2020-09-16 03:13:15.182335	2020-09-16 03:13:15.182335
4298352602374805827	6627591757152218867	2694133533970932102	2	2020-09-16 11:06:08.054389	2020-09-16 11:06:08.054389
2505438502570300843	2694133533970932102	6627591757152218867	2	2020-09-16 11:06:41.822437	2020-09-16 11:06:41.822437
6704349792416888516	8326186895181328012	4027022891871908826	1	2020-09-18 03:26:04.485837	2020-09-18 03:26:04.485837
8229171596351367859	6627591757152218867	4027022891871908826	1	2020-09-18 03:26:08.835865	2020-09-18 03:26:08.835865
2498726195242444084	329739458029569425	4027022891871908826	1	2020-09-18 03:26:12.234781	2020-09-18 03:26:12.234781
4954339106109017638	3935501584199374491	8544651895195766841	2	2020-09-03 03:50:40.250881	2020-09-03 03:50:40.250881
5202368778360731312	8544651895195766841	3935501584199374491	2	2020-09-03 08:16:43.271533	2020-09-03 08:16:43.271533
3405315768453215006	3455115140489977330	4027022891871908826	1	2020-09-18 03:26:18.883245	2020-09-18 03:26:18.883245
4481957491784216033	188137472857428845	4027022891871908826	1	2020-09-18 03:26:23.115747	2020-09-18 03:26:23.115747
1039828909801558192	5271470258722089835	4027022891871908826	2	2020-09-18 03:25:55.367308	2020-09-18 03:25:55.367308
6217418534252172946	4027022891871908826	5271470258722089835	2	2020-09-18 03:31:13.85122	2020-09-18 03:31:13.85122
8622898969773532176	6309855246227066278	4027022891871908826	2	2020-09-18 03:26:00.509755	2020-09-18 03:26:00.509755
3877009247354282992	4027022891871908826	6309855246227066278	2	2020-09-18 03:31:16.753987	2020-09-18 03:31:16.753987
\.


--
-- Data for Name: gem_relateds; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.gem_relateds (grid, obj_id, gid, obj_type, modify_time, created_time) FROM stdin;
1881594210991824282	3984697827620770275	9169467747588979885	2	2020-09-08 08:10:15.830224	2020-09-08 08:10:15.830224
4378065724864204250	9075465603310523524	9169467747588979885	2	2020-09-08 08:10:15.830224	2020-09-08 08:10:15.830224
1148701655906874426	1862664629529357697	9169467747588979885	2	2020-09-08 08:10:15.830224	2020-09-08 08:10:15.830224
1618774672715550096	8791584697569536544	9169467747588979885	2	2020-09-08 08:10:15.830224	2020-09-08 08:10:15.830224
305009468803358787	3667706950380863445	9169467747588979885	2	2020-09-08 08:10:15.830224	2020-09-08 08:10:15.830224
6587325365534675742	3534513442598030609	9169467747588979885	2	2020-09-08 08:10:15.830224	2020-09-08 08:10:15.830224
7676934728031024378	900238616481714883	1194675840908803999	1	2020-09-08 08:10:15.830224	2020-09-08 08:10:15.830224
1000999346312527998	900238616481714883	3230165357958183206	1	2020-09-08 08:10:15.830224	2020-09-08 08:10:15.830224
\.


--
-- Data for Name: gems; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.gems (gid, gem_icon, gem_selected_material, gem_link_material, model_path, kind, modify_time, created_time) FROM stdin;
9169467747588979885	Texture2D'/Game/Commons/Textures/Gems/Gems_Axelcon'	/Game/Commons/Textures/Material/M_GemSwordSelectedMat	/Game/Commons/Textures/Material/M_LinkSwordMat	Class'/Script/TheForeAwakensCPlus.MGemSword'	1	2020-09-08 08:10:15.81988	2020-09-08 08:10:15.81988
1194675840908803999	Texture2D'/Game/Commons/Textures/Gems/Gems_ShieldGem'	/Game/Commons/Textures/Material/M_GemShieldSelectedMat	/Game/Commons/Textures/Material/M_LinkShieldMat	Class'/Script/TheForeAwakensCPlus.MGemShield'	2	2020-09-08 08:10:15.81988	2020-09-08 08:10:15.81988
3230165357958183206	Texture2D'/Game/Commons/Textures/Gems/Gems_GoldGem'	/Game/Commons/Textures/Material/M_GemGoldSelectedMat	/Game/Commons/Textures/Material/M_LinkGoldMat	Class'/Script/TheForeAwakensCPlus.MGemGold'	3	2020-09-08 08:10:15.81988	2020-09-08 08:10:15.81988
2401662050616474945	Texture2D'/Game/Commons/Textures/Gems/Gems_Managem'	/Game/Commons/Textures/Material/M_GemManaSelectedMat	/Game/Commons/Textures/Material/M_LinkManaMat	Class'/Script/TheForeAwakensCPlus.MGemMana'	3	2020-09-08 08:10:15.81988	2020-09-08 08:10:15.81988
8932747538749248592	Texture2D'/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_1'	/Game/Commons/Textures/Material/M_GemSwordSelectedMat	/Game/Commons/Textures/Material/M_LinkSwordMat	Class'/Script/TheForeAwakensCPlus.MGemSword'	1	2020-09-08 08:10:15.81988	2020-09-08 08:10:15.81988
4106114089184243024	Texture2D'/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_2'	/Game/Commons/Textures/Material/M_GemSwordSelectedMat	/Game/Commons/Textures/Material/M_LinkSwordMat	Class'/Script/TheForeAwakensCPlus.MGemSword'	1	2020-09-08 08:10:15.81988	2020-09-08 08:10:15.81988
6428662726616975724	Texture2D'/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_3'	/Game/Commons/Textures/Material/M_GemSwordSelectedMat	/Game/Commons/Textures/Material/M_LinkSwordMat	Class'/Script/TheForeAwakensCPlus.MGemSword'	1	2020-09-08 08:10:15.81988	2020-09-08 08:10:15.81988
1209330646817269805	Texture2D'/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_1'	/Game/Commons/Textures/Material/M_GemShieldSelectedMat	/Game/Commons/Textures/Material/M_LinkShieldMat	Class'/Script/TheForeAwakensCPlus.MGemShield'	2	2020-09-08 08:10:15.81988	2020-09-08 08:10:15.81988
8374855459150100114	Texture2D'/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_2'	/Game/Commons/Textures/Material/M_GemShieldSelectedMat	/Game/Commons/Textures/Material/M_LinkShieldMat	Class'/Script/TheForeAwakensCPlus.MGemShield'	2	2020-09-08 08:10:15.81988	2020-09-08 08:10:15.81988
8672005677990751628	Texture2D'/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_3'	/Game/Commons/Textures/Material/M_GemShieldSelectedMat	/Game/Commons/Textures/Material/M_LinkShieldMat	Class'/Script/TheForeAwakensCPlus.MGemShield'	2	2020-09-08 08:10:15.81988	2020-09-08 08:10:15.81988
\.


--
-- Data for Name: link_accounts; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.link_accounts (lid, uuid, account_type, modify_time, created_time) FROM stdin;
\.


--
-- Data for Name: mall_first_recharge_gift_package_assets; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.mall_first_recharge_gift_package_assets (id, obj_id, obj_type, sort_value, modify_time, created_time) FROM stdin;
3548967653955616651	4992027716423162178	1	0	2020-09-17 11:20:00.838615	2020-09-17 11:20:00.838615
7655174417722897367	6405999076881613336	1	0	2020-09-17 11:20:00.838615	2020-09-17 11:20:00.838615
4585374955781501845	7068673142731586830	1	0	2020-09-17 11:20:00.838615	2020-09-17 11:20:00.838615
4217679897282184702	1317622610439774139	4	0	2020-09-17 11:20:00.838615	2020-09-17 11:20:00.838615
4976897654619361816	7411409667987130823	4	0	2020-09-17 11:20:00.838615	2020-09-17 11:20:00.838615
2742455810440183273	6199336508882440580	4	0	2020-09-17 11:20:00.838615	2020-09-17 11:20:00.838615
\.


--
-- Data for Name: mall_first_recharge_gift_packages; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.mall_first_recharge_gift_packages (id, name, bg_url, bg_desc, asset_desc, price, product_number, status, modify_time, created_time) FROM stdin;
3955771936752353126	First Recharge Gift Package		FIRST REACHARGE BONUS GENEROUS REWARDS	Recharge any amount for the first time,you can get the following props for free	0.99000001	tfa_first_recharge_gift_package	1	2020-09-17 11:20:00.832064	2020-09-17 11:20:00.832064
\.


--
-- Data for Name: mall_gem_stores; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.mall_gem_stores (id, name, gem_count, first_buy_handsel, late_buy_handsel, price, product_number, status, modify_time, created_time) FROM stdin;
4154528543820663973	Fistful of Gems	200	200	40	0.99000001	tfa_200_gems_pack	1	2020-09-17 11:20:00.846567	2020-09-17 11:20:00.846567
3121390111053433109	Pile of Gems	1050	1050	210	4.98999977	tfa_1050_gems_pack	1	2020-09-17 11:20:00.846567	2020-09-17 11:20:00.846567
6034451182568067067	Small Pouch of Gems	2200	2200	440	9.98999977	tfa_2200_gems_pack	1	2020-09-17 11:20:00.846567	2020-09-17 11:20:00.846567
206881496299641140	Big Pouch of Gems	4600	4600	920	19.9899998	tfa_4600_gems_pack	1	2020-09-17 11:20:00.846567	2020-09-17 11:20:00.846567
189719114316527438	Few bags of Gems	12000	12000	2400	49.9900017	tfa_12000_gems_pack	1	2020-09-17 11:20:00.846567	2020-09-17 11:20:00.846567
5292890078569536349	Wagon of Gems	25000	25000	5000	99.9899979	tfa_25000_gems_pack	1	2020-09-17 11:20:00.846567	2020-09-17 11:20:00.846567
\.


--
-- Data for Name: mall_supply_station_assets; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.mall_supply_station_assets (id, sid, obj_id, obj_type, sort_value, modify_time, created_time) FROM stdin;
120980258784564721	6136516928928798574	5094541857486127965	1	0	2020-09-17 11:20:00.853715	2020-09-17 11:20:00.853715
7898935314677036643	1457295478306587191	8332058209941129769	2	0	2020-09-17 11:20:00.853715	2020-09-17 11:20:00.853715
1765023205838734941	5559999827200708112	2811896600466761862	1	0	2020-09-17 11:20:00.853715	2020-09-17 11:20:00.853715
\.


--
-- Data for Name: mall_supply_stations; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.mall_supply_stations (id, name, description, thumbnail, gem_count, valid_period_day, price, product_number, status, modify_time, created_time) FROM stdin;
6136516928928798574	30 Day Gem Supply	And log in every day to claim a total of 19,500 gems! 		2200	30	9.98999977	tfa_30_day_gem_supply	1	2020-09-17 11:20:00.848718	2020-09-17 11:20:00.848718
1457295478306587191	7 Day Speedup Supply	And log in every day to claim a total of 3,150 Speedup minutes! 		1050	7	4.98999977	tfa_7_day_speedup_supply	1	2020-09-17 11:20:00.848718	2020-09-17 11:20:00.848718
5559999827200708112	7 Day Food Supply	And log in every day to claim a total of 7,000,000 Foods! 		1050	7	4.98999977	tfa_7_day_food_supply	1	2020-09-17 11:20:00.848718	2020-09-17 11:20:00.848718
\.


--
-- Data for Name: mall_user_buy_gem_store_records; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.mall_user_buy_gem_store_records (rid, uuid, gsid, modify_time, created_time) FROM stdin;
\.


--
-- Data for Name: mall_user_buy_supply_station_records; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.mall_user_buy_supply_station_records (id, uuid, sid, expire_time, latest_receive_time, modify_time, created_time) FROM stdin;
\.


--
-- Data for Name: mall_user_first_recharge_gift_package_records; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.mall_user_first_recharge_gift_package_records (id, uuid, modify_time, created_time) FROM stdin;
\.


--
-- Data for Name: player_mount_equipments; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.player_mount_equipments (id, pid, uid, equipment_id, modify_time, created_time) FROM stdin;
7205113581034754185	900238616481714883	5252403087755855415	9075465603310523524	2020-09-10 03:36:52.257347	2020-09-10 03:36:52.257347
5588206057062235552	900238616481714883	5252403087755855415	3667706950380863445	2020-09-10 06:01:48.423872	2020-09-10 06:01:48.423872
142552511185475859	900238616481714883	2947423296548425905	3667706950380863445	2020-09-10 06:38:55.972086	2020-09-10 06:38:55.972086
4898709982786005446	900238616481714883	2947423296548425905	9075465603310523524	2020-09-10 06:39:00.761604	2020-09-10 06:39:00.761604
6386067737116107259	900238616481714883	2947423296548425905	6505506293171167408	2020-09-10 06:40:00.707437	2020-09-10 06:40:00.707437
9064009125896856239	900238616481714883	5601071935482074023	8791584697569536544	2020-09-08 08:10:28.000784	2020-09-08 08:10:28.000784
7471112162433196506	900238616481714883	5601071935482074023	3984697827620770275	2020-09-08 08:10:35.508655	2020-09-08 08:10:35.508655
597850366833802582	900238616481714883	5601071935482074023	6750612841634615393	2020-09-08 08:10:40.085281	2020-09-08 08:10:40.085281
5389584055126904212	900238616481714883	3455115140489977330	8791584697569536544	2020-09-08 08:50:53.592832	2020-09-08 08:50:53.592832
3017322972851535948	900238616481714883	3455115140489977330	3984697827620770275	2020-09-08 08:50:56.856957	2020-09-08 08:50:56.856957
1549005039224469912	900238616481714883	3455115140489977330	6750612841634615393	2020-09-08 08:51:04.720211	2020-09-08 08:51:04.720211
382536631924089585	900238616481714883	3455115140489977330	3534513442598030609	2020-09-08 08:51:50.101087	2020-09-08 08:51:50.101087
5564295652169151651	900238616481714883	9065103658303460186	3667706950380863445	2020-09-09 01:57:35.449688	2020-09-09 01:57:35.449688
5020888254611188345	900238616481714883	9065103658303460186	9075465603310523524	2020-09-09 01:57:38.148226	2020-09-09 01:57:38.148226
5338872969701340722	900238616481714883	9065103658303460186	6750612841634615393	2020-09-09 01:58:14.006301	2020-09-09 01:58:14.006301
6764568721243425066	900238616481714883	5601071935482074023	3534513442598030609	2020-09-09 02:28:00.988352	2020-09-09 02:28:00.988352
4838277152885660989	900238616481714883	5601071935482074023	3667706950380863445	2020-09-09 02:30:09.43302	2020-09-09 02:30:09.43302
4511561652808702441	900238616481714883	5601071935482074023	9075465603310523524	2020-09-09 02:31:07.870075	2020-09-09 02:31:07.870075
8315356449872835327	900238616481714883	806107515131523577	3667706950380863445	2020-09-10 02:43:44.635284	2020-09-10 02:43:44.635284
6067824102847317440	900238616481714883	806107515131523577	9075465603310523524	2020-09-10 02:43:49.486824	2020-09-10 02:43:49.486824
3916266125966841410	900238616481714883	806107515131523577	6505506293171167408	2020-09-10 02:56:02.027778	2020-09-10 02:56:02.027778
6673344426738328965	900238616481714883	806107515131523577	8791584697569536544	2020-09-10 02:57:51.316936	2020-09-10 02:57:51.316936
6180549212175204917	900238616481714883	806107515131523577	3534513442598030609	2020-09-10 02:58:15.237948	2020-09-10 02:58:15.237948
47529685394459852	900238616481714883	806107515131523577	3984697827620770275	2020-09-10 02:59:36.285955	2020-09-10 02:59:36.285955
6286230287494598923	900238616481714883	806107515131523577	5488916325371064141	2020-09-10 02:59:40.413018	2020-09-10 02:59:40.413018
7180523318524389876	900238616481714883	806107515131523577	6750612841634615393	2020-09-10 03:00:39.115884	2020-09-10 03:00:39.115884
3970173158186970190	900238616481714883	2612773216355420476	8791584697569536544	2020-09-10 03:30:36.531971	2020-09-10 03:30:36.531971
1705801935656307586	900238616481714883	2612773216355420476	6750612841634615393	2020-09-10 03:31:06.300166	2020-09-10 03:31:06.300166
1737586275622280191	900238616481714883	2612773216355420476	3984697827620770275	2020-09-10 03:31:09.389841	2020-09-10 03:31:09.389841
6112898080856943237	900238616481714883	7875265132999058905	3534513442598030609	2020-09-10 03:32:14.384243	2020-09-10 03:32:14.384243
651092178769370	900238616481714883	4305958976104284087	8791584697569536544	2020-09-10 06:12:21.663081	2020-09-10 06:12:21.663081
4457342463757977694	900238616481714883	4305958976104284087	3984697827620770275	2020-09-10 06:12:30.85904	2020-09-10 06:12:30.85904
1220770618163578073	900238616481714883	4305958976104284087	6750612841634615393	2020-09-10 06:12:35.489131	2020-09-10 06:12:35.489131
673573962470416816	900238616481714883	5252403087755855415	6505506293171167408	2020-09-10 03:34:13.808431	2020-09-10 03:34:13.808431
\.


--
-- Data for Name: players; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.players (pid, player_name, model_path, thumbnail, max_hp, attack_power, move_speed, max_mana, defense, animation_hit_delay, spawn_style_class, level, star_level, level_experience, is_default, modify_time, created_time) FROM stdin;
900238616481714883	default_name	Blueprint'/Game/Match3RPG/Blueprints/Units/Player/HeroWizardSM.HeroWizardSM_C'	Texture2D'/Game/Commons/Textures/ChatSystem/Icon/Icon_ChatSystemHead.Icon_ChatSystemHead	20	5	600	20	5	0.5	/Script/TheForeAwakensCPlus.MRunSpawn	1	1	120	2	2020-09-08 08:10:15.824968	2020-09-08 08:10:15.824968
\.


--
-- Data for Name: props_resources_categorys; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.props_resources_categorys (rid, name, count, thumbnail, description, kind, modify_time, created_time) FROM stdin;
4992027716423162178	50 Gems	50		Get 50 gems after use.	2	2020-09-17 11:20:00.836556	2020-09-17 11:20:00.836556
6405999076881613336	300 Golds	300		Get 300 golds after use.	1	2020-09-17 11:20:00.836556	2020-09-17 11:20:00.836556
7068673142731586830	200 VIP Points	200		Add 200 VIP Points to your character.	3	2020-09-17 11:20:00.836556	2020-09-17 11:20:00.836556
5094541857486127965	650 Gems	650		Get 650 gems after use.	2	2020-09-17 11:20:00.850266	2020-09-17 11:20:00.850266
2811896600466761862	50,000 Foods	20		Get 50,000 food after use.	4	2020-09-17 11:20:00.850266	2020-09-17 11:20:00.850266
\.


--
-- Data for Name: props_speed_up_categorys; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.props_speed_up_categorys (sid, name, thumbnail, description, speed_time, count, modify_time, created_time) FROM stdin;
8332058209941129769	5-Minute Building Speedup		Reduces the time of a Building Queue by 5minutes	300	90	2020-09-17 11:20:00.85189	2020-09-17 11:20:00.85189
\.


--
-- Data for Name: purchase_orders; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.purchase_orders (oid, obj_id, obj_type, uuid, hash, product_number, pay_platform, order_no, status, pay_time, price, request_receipt_data, response_receipt_data, deleted_time, modify_time, created_time) FROM stdin;
\.


--
-- Data for Name: server_lists; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.server_lists (slid, name, country_code, area, ip, port, server_type, state, modify_time, created_time) FROM stdin;
7817487988393814824	localhost-mache	86	ch	192.168.1.129	4434	2	0	2020-07-22 17:47:53.336092	2020-07-22 17:47:53.336092
\.


--
-- Data for Name: servers; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.servers (sid, server_number, name, ip, ports, person_count, modify_time, created_time) FROM stdin;
6829118970656486265	1001	localhost-mache	127.0.0.1	4433	6444	2020-06-09 15:07:37.216099	2020-06-09 15:07:37.216099
\.


--
-- Data for Name: skill_fight_relateds; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.skill_fight_relateds (id, obj_id, skill_id, cool_down, attack_power, mana_power, probability, level, level_experience, obj_type, modify_time, created_time) FROM stdin;
8038383485969862698	7367515832694383145	28423341241102037	3	20	10	0	0	0	2	2020-09-15 08:28:36.073391	2020-09-15 08:28:36.073391
5377242200877481441	2230968894271688632	6834593558916891530	3	0	10	0	1	1	2	2020-09-15 08:28:36.073391	2020-09-15 08:28:36.073391
7917881644006967153	549236052085417001	3858293095703469656	3	20	10	0	0	0	2	2020-09-17 08:34:36.693832	2020-09-17 08:34:36.693832
378747250763778786	6530684588090682561	7044579216040858314	3	0	10	0	1	1	2	2020-09-17 08:34:36.693832	2020-09-17 08:34:36.693832
6426464684466655122	7705326282310627813	3748540917415927219	3	20	10	0	0	0	2	2020-09-17 10:38:50.80985	2020-09-17 10:38:50.80985
67911417519146049	544238997007077338	2024195177425183112	3	0	10	0	1	1	2	2020-09-17 10:38:50.80985	2020-09-17 10:38:50.80985
1953461439881874438	1317622610439774139	52624050549968620	3	20	10	0	0	0	2	2020-09-17 11:20:00.843982	2020-09-17 11:20:00.843982
4534420489617731287	6199336508882440580	2856724125974468978	3	0	10	0	1	1	2	2020-09-17 11:20:00.843982	2020-09-17 11:20:00.843982
4107300396048681876	6905828721094891599	7568918151083537934	3	0	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
1970189105362291886	6946808681194466747	6628775941247734999	3	0	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
1574835480617758152	7775215474871160888	8488922864783419063	3	0	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
8848599715786343556	338676047550973005	6628775941247734999	3	0	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
8617208802824992734	8078812079154036494	7619758325740407970	3	0	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
7374713158265743113	8078812079154036494	7904177909314920963	3	0	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
3877919355346881667	5434195190315914010	6099340297956815248	3	0	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
1066977343245826973	5434195190315914010	7904177909314920963	3	0	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
1864899825361328638	5683408629523021734	6628775941247734999	3	0	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
7715656580884055674	2647844488543154810	6628775941247734999	3	0	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
6940890216251665393	3779333134542240560	4875337034647119960	3	0	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
2354492925281860891	9088913670112212976	6628775941247734999	3	0	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
5165756187606424368	9088913670112212976	7904177909314920963	3	0	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
6595772273508811707	7340739757772068518	8956446547459299716	3	0	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
749075949803939930	75218728476491326	4202585224775900740	3	0	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
2964672202836823669	4278179493359130662	7273328100961457444	3	0	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
8669065853454649352	922787162695561417	7167357347664678419	3	10	0	1	0	0	3	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
8683137782568399584	3984697827620770275	7167357347664678419	2	0	5	0	1	1	2	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
2587919902736535548	9075465603310523524	7167357347664678419	2	0	5	0	1	1	2	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
3136653409703243172	1862664629529357697	7167357347664678419	2	0	5	0	1	1	2	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
5312971509625595802	8791584697569536544	2815262303215164842	2	0	5	0	1	1	2	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
1241175426596970779	3667706950380863445	7912874777660579789	2	0	5	0	1	1	2	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
3665771877855058790	3534513442598030609	1046902652484595077	2	0	5	0	1	1	2	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
2166439826698030230	900238616481714883	1097667518054812201	2	0	5	0	1	1	1	2020-09-08 08:10:15.827952	2020-09-08 08:10:15.827952
1596554775040541462	8349886201892374001	6381737790350137019	3	20	10	0	0	0	2	2020-09-17 07:33:50.523312	2020-09-17 07:33:50.523312
3377271015500614018	6615567943590654816	1084029379272793030	3	0	10	0	1	1	2	2020-09-17 07:33:50.523312	2020-09-17 07:33:50.523312
5266777918193625792	5515803116356807042	651709731077204918	3	20	10	0	0	0	2	2020-09-17 08:42:08.262358	2020-09-17 08:42:08.262358
1148406873863448797	8712897837927593025	440008188875722492	3	0	10	0	1	1	2	2020-09-17 08:42:08.262358	2020-09-17 08:42:08.262358
3725360622701965897	6678320473774019377	7512897863577743994	3	20	10	0	0	0	2	2020-09-17 10:52:28.251877	2020-09-17 10:52:28.251877
7610759595559102476	7775730202917710895	2791054446156104917	3	0	10	0	1	1	2	2020-09-17 10:52:28.251877	2020-09-17 10:52:28.251877
\.


--
-- Data for Name: skills; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.skills (id, thumbnail, skill_name, skill_description, model_path, cool_down, attack_power, mana_power, level, level_experience, modify_time, created_time) FROM stdin;
1097667518054812201	Texture2D'/Game/Commons/Textures/Skill/Icon_Refresh.Icon_Refresh'	Refresh Board	Refreshes the board with new gems.	Class'/Script/TheForeAwakensCPlus.MRefreshBoard'	2	0	5	1	1	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
6069086799951353869	Texture2D'/Game/Commons/Textures/Skill/Skill_Treatment.Skill_Treatment'	Blood Return	Focuses and performs a precise attack.	Class'/Script/TheForeAwakensCPlus.MBloodReturn'	0	6	1	1	1	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
7167357347664678419	Texture2D'/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_2.Equipment_Weapon_2'	Three Combo	A weak attack performed by an average fighter.	Class'/Script/TheForeAwakensCPlus.MThreeCombo'	3	10	0	0	0	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
2815262303215164842	Texture2D'/Game/Commons/Textures/Skill/Skill_1.Skill_1'	Single Attack	A weak attack performed by an average fighter.	Class'/Script/TheForeAwakensCPlus.MSingleAttack'	3	15	0	0	0	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
7912874777660579789	Texture2D'/Game/Commons/Textures/Skill/Skill_3.Skill_3'	Jump Attack	Breaks 5 shield gems on the board.	Class'/Script/TheForeAwakensCPlus.MJumpAttack'	3	0	0	0	0	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
1046902652484595077	Texture2D'/Game/Commons/Textures/Skill/Skill_Mobs_1.Skill_Mobs_1'	Five Combo	Breaks 5 shield gems on the board.	Class'/Script/TheForeAwakensCPlus.MFiveCombo'	3	0	0	0	0	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
7568918151083537934	Texture2D'/Game/Commons/Textures/Skill/Skill_Mobs_1.Skill_Mobs_1'	Five Combo	Breaks 5 shield gems on the board.	Class'/Script/TheForeAwakensCPlus.MBatSendBullet'	3	0	0	0	0	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
8488922864783419063	Texture2D'/Game/Commons/Textures/Skill/Skill_Mobs_1.Skill_Mobs_1'	Five Combo	Breaks 5 shield gems on the board.	Class'/Script/TheForeAwakensCPlus.MEvilmageSendBlueFireBall'	3	0	0	0	0	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
6099340297956815248	Texture2D'/Game/Commons/Textures/Skill/Skill_Mobs_1.Skill_Mobs_1'	Five Combo	Breaks 5 shield gems on the board.	Class'/Script/TheForeAwakensCPlus.MGhoulFesteringSkill'	3	0	0	0	0	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
7619758325740407970	Texture2D'/Game/Commons/Textures/Skill/Skill_Mobs_1.Skill_Mobs_1'	Five Combo	Breaks 5 shield gems on the board.	Class'/Script/TheForeAwakensCPlus.MGhoulSkill'	3	0	0	0	0	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
9127299102076772673	Texture2D'/Game/Commons/Textures/Skill/Skill_Mobs_1.Skill_Mobs_1'	Five Combo	Breaks 5 shield gems on the board.	Class'/Script/TheForeAwakensCPlus.MOutTheBlue'	3	0	0	0	0	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
4875337034647119960	Texture2D'/Game/Commons/Textures/Skill/Skill_Mobs_1.Skill_Mobs_1'	Five Combo	Breaks 5 shield gems on the board.	Class'/Script/TheForeAwakensCPlus.MSendBullet'	3	0	0	0	0	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
7904177909314920963	Texture2D'/Game/Commons/Textures/Skill/Skill_Mobs_1.Skill_Mobs_1'	Five Combo	Breaks 5 shield gems on the board.	Class'/Script/TheForeAwakensCPlus.MShieldBreak'	3	0	0	0	0	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
8956446547459299716	Texture2D'/Game/Commons/Textures/Skill/Skill_Mobs_1.Skill_Mobs_1'	Five Combo	Breaks 5 shield gems on the board.	Class'/Script/TheForeAwakensCPlus.MSkeletonSkill'	3	0	0	0	0	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
7273328100961457444	Texture2D'/Game/Commons/Textures/Skill/Skill_Mobs_1.Skill_Mobs_1'	Five Combo	Breaks 5 shield gems on the board.	Class'/Script/TheForeAwakensCPlus.MSpiderSendLaster'	3	0	0	0	0	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
6628775941247734999	Texture2D'/Game/Commons/Textures/Skill/Skill_Mobs_1.Skill_Mobs_1'	Five Combo	Breaks 5 shield gems on the board.	Class'/Script/TheForeAwakensCPlus.MStrongAttack'	3	0	0	0	0	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
4202585224775900740	Texture2D'/Game/Commons/Textures/Skill/Skill_Mobs_1.Skill_Mobs_1'	Five Combo	Breaks 5 shield gems on the board.	Class'/Script/TheForeAwakensCPlus.MWeakAttack'	3	0	0	0	0	2020-09-08 08:10:15.811496	2020-09-08 08:10:15.811496
28423341241102037		Blazing		Class'/Script/TheForeAwakensCPlus.MThreeCombo'	3	20	10	0	0	2020-09-15 08:28:36.071549	2020-09-15 08:28:36.071549
6834593558916891530		Eternal Shield		Class'/Script/TheForeAwakensCPlus.MShieldSkill'	3	0	10	1	1	2020-09-15 08:28:36.071549	2020-09-15 08:28:36.071549
6381737790350137019		Blazing		Class'/Script/TheForeAwakensCPlus.MThreeCombo'	3	20	10	0	0	2020-09-17 07:33:50.521874	2020-09-17 07:33:50.521874
1084029379272793030		Eternal Shield		Class'/Script/TheForeAwakensCPlus.MShieldSkill'	3	0	10	1	1	2020-09-17 07:33:50.521874	2020-09-17 07:33:50.521874
3858293095703469656		Blazing		Class'/Script/TheForeAwakensCPlus.MThreeCombo'	3	20	10	0	0	2020-09-17 08:34:36.691859	2020-09-17 08:34:36.691859
7044579216040858314		Eternal Shield		Class'/Script/TheForeAwakensCPlus.MShieldSkill'	3	0	10	1	1	2020-09-17 08:34:36.691859	2020-09-17 08:34:36.691859
651709731077204918		Blazing		Class'/Script/TheForeAwakensCPlus.MThreeCombo'	3	20	10	0	0	2020-09-17 08:42:08.260289	2020-09-17 08:42:08.260289
440008188875722492		Eternal Shield		Class'/Script/TheForeAwakensCPlus.MShieldSkill'	3	0	10	1	1	2020-09-17 08:42:08.260289	2020-09-17 08:42:08.260289
3748540917415927219		Blazing		Class'/Script/TheForeAwakensCPlus.MThreeCombo'	3	20	10	0	0	2020-09-17 10:38:50.808202	2020-09-17 10:38:50.808202
2024195177425183112		Eternal Shield		Class'/Script/TheForeAwakensCPlus.MShieldSkill'	3	0	10	1	1	2020-09-17 10:38:50.808202	2020-09-17 10:38:50.808202
7512897863577743994		Blazing		Class'/Script/TheForeAwakensCPlus.MThreeCombo'	3	20	10	0	0	2020-09-17 10:52:28.250413	2020-09-17 10:52:28.250413
2791054446156104917		Eternal Shield		Class'/Script/TheForeAwakensCPlus.MShieldSkill'	3	0	10	1	1	2020-09-17 10:52:28.250413	2020-09-17 10:52:28.250413
52624050549968620		Blazing		Class'/Script/TheForeAwakensCPlus.MThreeCombo'	3	20	10	0	0	2020-09-17 11:20:00.842201	2020-09-17 11:20:00.842201
2856724125974468978		Eternal Shield		Class'/Script/TheForeAwakensCPlus.MShieldSkill'	3	0	10	1	1	2020-09-17 11:20:00.842201	2020-09-17 11:20:00.842201
\.


--
-- Data for Name: user_assets; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.user_assets (asid, uid, golds, diamonds, modify_time, created_time) FROM stdin;
6108521602025084352	2318448884852593867	10000	10000	2020-09-08 11:08:52.042273	2020-09-08 11:08:52.042273
2519025388967137357	854473010740327354	10000	10000	2020-09-08 11:22:52.634961	2020-09-08 11:22:52.634961
2038246323581166570	3694901305134076376	0	0	2020-09-08 11:41:41.765753	2020-09-08 11:41:41.765753
7691632398724088642	1193295810120197710	10000	10000	2020-09-08 11:41:44.575887	2020-09-08 11:41:44.575887
8634001345116107116	9065103658303460186	8270	10000	2020-09-09 01:49:33.886297	2020-09-09 01:49:33.886297
7182612392764098983	4305958976104284087	8270	10000	2020-09-10 06:12:07.621192	2020-09-10 06:12:07.621192
7474161644295361888	4833729286789198324	10000	10000	2020-09-10 07:17:11.881891	2020-09-10 07:17:11.881891
3891783104090589000	8587893535813273098	0	0	2020-09-10 10:03:13.623475	2020-09-10 10:03:13.623475
5896528827421052222	5601071935482074023	8270	10000	2020-09-08 08:10:16.515446	2020-09-08 08:10:16.515446
2113419598408029843	4202637381044109981	10000	10000	2020-09-09 04:14:39.317757	2020-09-09 04:14:39.317757
5729483665710170638	2302510692996666299	10000	10000	2020-09-09 11:26:35.436115	2020-09-09 11:26:35.436115
1584529948834834285	6575428629280265870	0	0	2020-09-10 10:03:13.631065	2020-09-10 10:03:13.631065
8291324434064464547	1041955479368729000	0	0	2020-09-11 01:50:36.251302	2020-09-11 01:50:36.251302
1856750295408430316	1981557320191779142	0	0	2020-09-11 01:50:36.266492	2020-09-11 01:50:36.266492
6870332020316035419	5429881535936798637	10000	10000	2020-09-14 10:20:24.277598	2020-09-14 10:20:24.277598
5268498858825492236	7659254709327742843	8270	10000	2020-09-10 03:22:08.368096	2020-09-10 03:22:08.368096
8094133371063028685	7875265132999058905	8270	10000	2020-09-10 03:32:01.008575	2020-09-10 03:32:01.008575
7653604575603083602	7375716747774944843	8270	10000	2020-09-15 06:12:38.63696	2020-09-15 06:12:38.63696
6697117112210781779	6457364932904658384	0	0	2020-09-15 10:41:04.073455	2020-09-15 10:41:04.073455
8899881990244519598	8603229685798689003	10000	10000	2020-09-16 06:37:22.736904	2020-09-16 06:37:22.736904
4867714395761624219	2138533638807468853	10000	10000	2020-09-17 06:10:37.008079	2020-09-17 06:10:37.008079
2931244508718999123	5694730814590643057	0	0	2020-09-17 06:41:40.144757	2020-09-17 06:41:40.144757
639025903680875209	3871910907348016407	10000	10000	2020-09-17 07:32:16.61978	2020-09-17 07:32:16.61978
8558637970422566213	7037768559378452897	0	0	2020-09-18 02:12:44.939568	2020-09-18 02:12:44.939568
1467016613527847478	8565050772142024576	0	0	2020-09-18 02:22:53.309382	2020-09-18 02:22:53.309382
8604093986316895734	8699691129036934747	10000	10000	2020-09-18 03:00:53.024621	2020-09-18 03:00:53.024621
1822900431331529894	9037228755693796387	0	0	2020-09-18 03:16:58.748084	2020-09-18 03:16:58.748084
2930526919135921621	2838704115976202491	10000	10000	2020-09-08 08:16:11.916299	2020-09-08 08:16:11.916299
878026420233464033	2714470504778644006	10000	10000	2020-09-08 08:46:37.975038	2020-09-08 08:46:37.975038
4293562519158275359	1775585148607179624	0	0	2020-09-08 08:55:49.537949	2020-09-08 08:55:49.537949
4811790293554456495	9069012371734241364	0	0	2020-09-08 08:55:49.559309	2020-09-08 08:55:49.559309
6856526769511775557	2598008582423324298	0	0	2020-09-08 08:55:49.565211	2020-09-08 08:55:49.565211
1479998267005154899	5394771594751493691	0	0	2020-09-08 08:55:49.570919	2020-09-08 08:55:49.570919
2912584393709954119	5353313573339177523	0	0	2020-09-08 08:55:49.575759	2020-09-08 08:55:49.575759
8674206906434847239	5271470258722089835	0	0	2020-09-08 09:09:00.201487	2020-09-08 09:09:00.201487
6442532343002652707	6309855246227066278	0	0	2020-09-08 09:09:00.223674	2020-09-08 09:09:00.223674
1004329192731853447	8326186895181328012	0	0	2020-09-08 09:09:00.229257	2020-09-08 09:09:00.229257
522872694493543250	6627591757152218867	0	0	2020-09-08 09:09:00.234443	2020-09-08 09:09:00.234443
546115423129081282	329739458029569425	0	0	2020-09-08 09:09:00.237709	2020-09-08 09:09:00.237709
6359730702801817543	6219686592090773906	0	0	2020-09-08 11:22:50.18821	2020-09-08 11:22:50.18821
2219827883255464066	6855857258826464301	0	0	2020-09-08 11:25:06.675688	2020-09-08 11:25:06.675688
6202445828583662619	3824108440689794940	10000	10000	2020-09-08 09:00:41.520728	2020-09-08 09:00:41.520728
8958004837237724135	7731757567175278668	10000	10000	2020-09-08 11:25:10.193592	2020-09-08 11:25:10.193592
1761175939057142085	3170318566048626046	0	0	2020-09-09 01:49:31.055423	2020-09-09 01:49:31.055423
2002012307748668460	276871354416890129	10000	10000	2020-09-08 09:13:25.012048	2020-09-08 09:13:25.012048
4688309193074343253	978896616369573409	10000	10000	2020-09-09 04:07:12.531873	2020-09-09 04:07:12.531873
5024091782962721283	9055950420423069060	10000	10000	2020-09-09 11:27:22.847029	2020-09-09 11:27:22.847029
2118028020950712067	806107515131523577	8270	10000	2020-09-09 06:02:41.179819	2020-09-09 06:02:41.179819
6613025798839208707	2947423296548425905	8270	10000	2020-09-10 06:37:26.144455	2020-09-10 06:37:26.144455
6593767428304202332	5986398665897825204	0	0	2020-09-10 09:50:55.497949	2020-09-10 09:50:55.497949
7868307020117873533	5223911966781382373	0	0	2020-09-10 09:50:55.510754	2020-09-10 09:50:55.510754
3548003054985564212	4959061375679117082	0	0	2020-09-10 10:51:28.617908	2020-09-10 10:51:28.617908
5896463777304349889	8147385450836295281	0	0	2020-09-10 10:51:28.639593	2020-09-10 10:51:28.639593
1107343994592362432	2612773216355420476	8270	10000	2020-09-10 03:30:17.017603	2020-09-10 03:30:17.017603
3026644304033421305	8799391418637662520	10000	10000	2020-09-14 08:02:33.894112	2020-09-14 08:02:33.894112
7906752776887870815	188137472857428845	10000	10000	2020-09-14 11:24:16.348836	2020-09-14 11:24:16.348836
387508740081731356	8281437102882339157	10000	10000	2020-09-15 09:04:28.988799	2020-09-15 09:04:28.988799
8188949550451984099	5252403087755855415	8270	10000	2020-09-10 03:33:55.038964	2020-09-10 03:33:55.038964
1266868287198875070	2694133533970932102	10000	10000	2020-09-16 02:43:25.246742	2020-09-16 02:43:25.246742
6728483565402890654	681771573559560332	10000	10000	2020-09-17 02:33:28.126259	2020-09-17 02:33:28.126259
9182048872378925939	7952244182381372668	0	0	2020-09-17 06:38:53.265833	2020-09-17 06:38:53.265833
3400555022741821895	5918735720551767331	0	0	2020-09-17 06:42:22.332484	2020-09-17 06:42:22.332484
2295450389133157075	7841791345558273926	0	0	2020-09-17 10:56:10.108799	2020-09-17 10:56:10.108799
2883777398022840789	4177492899450748665	0	0	2020-09-18 02:18:40.628918	2020-09-18 02:18:40.628918
5129241661494040397	8571848086878800496	0	0	2020-09-18 02:19:32.703148	2020-09-18 02:19:32.703148
4362264677276675192	7530724731559347298	0	0	2020-09-18 02:20:51.260607	2020-09-18 02:20:51.260607
1737477872686798378	9141776149995123164	10000	10000	2020-09-18 02:26:01.224557	2020-09-18 02:26:01.224557
7033995544619474084	2448925535796042305	0	0	2020-09-18 02:28:14.428779	2020-09-18 02:28:14.428779
8012460759810019649	4027022891871908826	10000	10000	2020-09-18 03:10:54.033975	2020-09-18 03:10:54.033975
4975781765964999084	8352742620538922856	10000	10000	2020-09-18 03:29:54.102827	2020-09-18 03:29:54.102827
6645109460318074072	3707743263319537005	10000	10000	2020-09-08 08:11:39.262375	2020-09-08 08:11:39.262375
4934293408064489322	4365732954452568148	10000	10000	2020-09-08 08:39:57.243368	2020-09-08 08:39:57.243368
2047143175165240899	3455115140489977330	8270	10000	2020-09-08 08:50:26.407567	2020-09-08 08:50:26.407567
\.


--
-- Data for Name: user_chat_unread_counts; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.user_chat_unread_counts (ucid, uuid_s, uuid_d, latest_timestamp, unread_count, modify_time, created_time) FROM stdin;
404854608359423108	8147385450836295281	4959061375679117082	1599736155	0	2020-09-10 11:09:15.341998	2020-09-10 10:52:13.703445
7182018171041428777	188137472857428845	6309855246227066278	0	2	2020-09-15 08:35:55.341828	2020-09-15 05:44:58.086371
3074792572718174543	1981557320191779142	1041955479368729000	1599790376	0	2020-09-11 02:12:56.291651	2020-09-11 01:51:45.087854
7957217213667623491	5271470258722089835	3455115140489977330	1599788882	1	2020-09-11 02:14:50.641081	2020-09-10 06:02:01.222452
702921060151966418	8326186895181328012	3455115140489977330	0	1	2020-09-11 02:17:19.385737	2020-09-11 02:17:19.385737
3564499453531594201	3455115140489977330	5271470258722089835	1599792410	0	2020-09-11 02:46:50.720559	2020-09-10 06:02:01.222452
6833584075781589967	6309855246227066278	188137472857428845	1600161684515	0	2020-09-15 09:21:24.515069	2020-09-15 02:12:52.240756
3985193442976160921	10006560	188137472857428845	0	1	2020-09-16 02:45:04.580754	2020-09-16 02:45:04.580754
1686605772768864802	10006560	3455115140489977330	0	4	2020-09-16 02:46:03.82292	2020-09-16 02:45:15.476664
8694110922298529116	329739458029569425	6627591757152218867	1599730867	0	2020-09-10 09:41:07.477067	2020-09-10 09:40:15.467293
2579424947430693748	329739458029569425	8326186895181328012	1599730897	0	2020-09-10 09:41:37.578661	2020-09-10 09:41:25.848453
5445155031394128682	6309855246227066278	5429881535936798637	0	1	2020-09-14 11:03:07.321531	2020-09-14 11:03:07.321531
8783431413890282035	5223911966781382373	5986398665897825204	1599732505	0	2020-09-10 10:08:25.336489	2020-09-10 09:52:06.416147
4458521470798816418	6575428629280265870	8587893535813273098	1599732704	0	2020-09-10 10:11:44.235563	2020-09-10 10:05:49.381742
5033192625648356175	6627591757152218867	188137472857428845	0	7	2020-09-15 02:35:15.175779	2020-09-14 11:27:40.738466
3766073232853365613	8587893535813273098	6575428629280265870	1599734762	0	2020-09-10 10:46:02.301595	2020-09-10 10:08:40.920911
3162038103110805772	2694133533970932102	3455115140489977330	1600251392588	0	2020-09-16 10:16:32.588595	2020-09-16 10:04:56.835549
6686724095267668249	4298352602374805827	6627591757152218867	0	1	2020-09-16 11:07:29.734296	2020-09-16 11:07:29.734296
7855155595450309576	5986398665897825204	3455115140489977330	0	2	2020-09-15 06:17:14.5567	2020-09-15 06:11:35.594421
4181135417704373512	2694133533970932102	6627591757152218867	1600254749254	0	2020-09-16 11:12:29.25451	2020-09-16 11:12:28.90034
3461685524671037402	6627591757152218867	2694133533970932102	0	3	2020-09-16 11:15:08.983035	2020-09-16 11:12:46.995692
4940575024113299620	3455115140489977330	5986398665897825204	1600155346419	0	2020-09-15 07:35:46.419025	2020-09-11 09:22:32.963213
5627089391388915802	3455115140489977330	2694133533970932102	1600251605706	2	2020-09-17 06:18:07.666342	2020-09-16 02:47:05.597638
8144147384377679877	5271470258722089835	2694133533970932102	0	3	2020-09-17 06:18:19.674688	2020-09-16 03:13:33.189666
1639787533071511407	188137472857428845	2694133533970932102	0	3	2020-09-17 06:23:41.494747	2020-09-16 03:12:49.809079
4566528375279682303	4027022891871908826	5271470258722089835	1600402663476	0	2020-09-18 04:17:43.476059	2020-09-18 04:17:12.796558
\.


--
-- Data for Name: user_equipments; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.user_equipments (id, eid, uid, modify_time, created_time) FROM stdin;
1247215845892907087	6750612841634615393	5271470258722089835	2020-09-08 09:09:00.201487	2020-09-08 09:09:00.201487
3723260119550716306	3984697827620770275	5271470258722089835	2020-09-08 09:09:00.201487	2020-09-08 09:09:00.201487
8358783406599412379	8791584697569536544	5271470258722089835	2020-09-08 09:09:00.201487	2020-09-08 09:09:00.201487
2068872396738332841	6750612841634615393	6309855246227066278	2020-09-08 09:09:00.223674	2020-09-08 09:09:00.223674
2300494640953829197	3984697827620770275	6309855246227066278	2020-09-08 09:09:00.223674	2020-09-08 09:09:00.223674
8373802712919366556	8791584697569536544	6309855246227066278	2020-09-08 09:09:00.223674	2020-09-08 09:09:00.223674
3245368200190411845	6750612841634615393	8326186895181328012	2020-09-08 09:09:00.229257	2020-09-08 09:09:00.229257
1358660192628505020	3984697827620770275	8326186895181328012	2020-09-08 09:09:00.229257	2020-09-08 09:09:00.229257
197624016421152625	8791584697569536544	8326186895181328012	2020-09-08 09:09:00.229257	2020-09-08 09:09:00.229257
1166798408477541600	6750612841634615393	6627591757152218867	2020-09-08 09:09:00.234443	2020-09-08 09:09:00.234443
2553282123174590158	3984697827620770275	6627591757152218867	2020-09-08 09:09:00.234443	2020-09-08 09:09:00.234443
7758773183797887438	8791584697569536544	6627591757152218867	2020-09-08 09:09:00.234443	2020-09-08 09:09:00.234443
5901356115478487522	6750612841634615393	329739458029569425	2020-09-08 09:09:00.237709	2020-09-08 09:09:00.237709
9004299887640836619	3984697827620770275	329739458029569425	2020-09-08 09:09:00.237709	2020-09-08 09:09:00.237709
4835662251060594960	8791584697569536544	329739458029569425	2020-09-08 09:09:00.237709	2020-09-08 09:09:00.237709
3492187041914189780	6750612841634615393	2318448884852593867	2020-09-08 11:08:52.042273	2020-09-08 11:08:52.042273
1824633362149088427	3984697827620770275	2318448884852593867	2020-09-08 11:08:52.042273	2020-09-08 11:08:52.042273
2274430464690887790	8791584697569536544	2318448884852593867	2020-09-08 11:08:52.042273	2020-09-08 11:08:52.042273
2483917569373770121	6750612841634615393	3694901305134076376	2020-09-08 11:41:41.765753	2020-09-08 11:41:41.765753
6640044349248175234	3984697827620770275	3694901305134076376	2020-09-08 11:41:41.765753	2020-09-08 11:41:41.765753
2702284464956071551	8791584697569536544	3694901305134076376	2020-09-08 11:41:41.765753	2020-09-08 11:41:41.765753
649822672647983583	6750612841634615393	1193295810120197710	2020-09-08 11:41:44.575887	2020-09-08 11:41:44.575887
2165609835889759385	3984697827620770275	1193295810120197710	2020-09-08 11:41:44.575887	2020-09-08 11:41:44.575887
3754287976646088501	8791584697569536544	1193295810120197710	2020-09-08 11:41:44.575887	2020-09-08 11:41:44.575887
5288434059329641048	3667706950380863445	5601071935482074023	2020-09-09 02:27:46.560698	2020-09-09 02:27:46.560698
8610396178400447768	3534513442598030609	5601071935482074023	2020-09-09 02:27:49.939491	2020-09-09 02:27:49.939491
4817186095716791613	6750612841634615393	806107515131523577	2020-09-09 06:02:41.179819	2020-09-09 06:02:41.179819
852546139511820312	3984697827620770275	806107515131523577	2020-09-09 06:02:41.179819	2020-09-09 06:02:41.179819
4090773615981218865	8791584697569536544	806107515131523577	2020-09-09 06:02:41.179819	2020-09-09 06:02:41.179819
8489855066827226972	6750612841634615393	7659254709327742843	2020-09-10 03:22:08.368096	2020-09-10 03:22:08.368096
7494756194213969845	3984697827620770275	7659254709327742843	2020-09-10 03:22:08.368096	2020-09-10 03:22:08.368096
5959304506670332437	8791584697569536544	7659254709327742843	2020-09-10 03:22:08.368096	2020-09-10 03:22:08.368096
8738765257312036449	6505506293171167408	7875265132999058905	2020-09-10 03:32:56.133643	2020-09-10 03:32:56.133643
1133788080986294065	5488916325371064141	7875265132999058905	2020-09-10 03:32:57.343043	2020-09-10 03:32:57.343043
8213444334259709011	9075465603310523524	7875265132999058905	2020-09-10 03:32:59.140927	2020-09-10 03:32:59.140927
5531637080911399981	1862664629529357697	7875265132999058905	2020-09-10 03:33:00.275632	2020-09-10 03:33:00.275632
3484756374535337719	3534513442598030609	4305958976104284087	2020-09-10 06:13:43.705752	2020-09-10 06:13:43.705752
311422896758344488	1862664629529357697	4305958976104284087	2020-09-10 06:13:51.4956	2020-09-10 06:13:51.4956
3917708193954184494	9075465603310523524	4305958976104284087	2020-09-10 06:13:53.947485	2020-09-10 06:13:53.947485
7021255013690807040	6505506293171167408	4305958976104284087	2020-09-10 06:13:55.753625	2020-09-10 06:13:55.753625
3535736718089349904	6750612841634615393	4833729286789198324	2020-09-10 07:17:11.881891	2020-09-10 07:17:11.881891
5213241655311725298	3984697827620770275	4833729286789198324	2020-09-10 07:17:11.881891	2020-09-10 07:17:11.881891
5960566672668902824	8791584697569536544	4833729286789198324	2020-09-10 07:17:11.881891	2020-09-10 07:17:11.881891
8085120963548318297	6750612841634615393	1041955479368729000	2020-09-11 01:50:36.251302	2020-09-11 01:50:36.251302
4417808903950420414	3984697827620770275	1041955479368729000	2020-09-11 01:50:36.251302	2020-09-11 01:50:36.251302
6592045544260072780	8791584697569536544	1041955479368729000	2020-09-11 01:50:36.251302	2020-09-11 01:50:36.251302
8284802043573524239	6750612841634615393	1981557320191779142	2020-09-11 01:50:36.266492	2020-09-11 01:50:36.266492
4076186863654033146	3984697827620770275	1981557320191779142	2020-09-11 01:50:36.266492	2020-09-11 01:50:36.266492
4709973020455459725	8791584697569536544	1981557320191779142	2020-09-11 01:50:36.266492	2020-09-11 01:50:36.266492
737154495230937410	6750612841634615393	7375716747774944843	2020-09-15 06:12:38.63696	2020-09-15 06:12:38.63696
2636131209075520085	3984697827620770275	7375716747774944843	2020-09-15 06:12:38.63696	2020-09-15 06:12:38.63696
1277736435186414666	8791584697569536544	7375716747774944843	2020-09-15 06:12:38.63696	2020-09-15 06:12:38.63696
526986599506647636	6750612841634615393	2694133533970932102	2020-09-16 02:43:25.246742	2020-09-16 02:43:25.246742
4691980518240859603	3984697827620770275	2694133533970932102	2020-09-16 02:43:25.246742	2020-09-16 02:43:25.246742
3380580904954476002	8791584697569536544	2694133533970932102	2020-09-16 02:43:25.246742	2020-09-16 02:43:25.246742
8018174586514414074	6750612841634615393	7952244182381372668	2020-09-17 06:38:53.265833	2020-09-17 06:38:53.265833
162820601306036583	3984697827620770275	7952244182381372668	2020-09-17 06:38:53.265833	2020-09-17 06:38:53.265833
644301939110927135	8791584697569536544	7952244182381372668	2020-09-17 06:38:53.265833	2020-09-17 06:38:53.265833
5715279612866758269	6750612841634615393	7841791345558273926	2020-09-17 10:56:10.108799	2020-09-17 10:56:10.108799
8656476394950237230	3984697827620770275	7841791345558273926	2020-09-17 10:56:10.108799	2020-09-17 10:56:10.108799
2705717576159895731	8791584697569536544	7841791345558273926	2020-09-17 10:56:10.108799	2020-09-17 10:56:10.108799
2079034727524845711	6750612841634615393	9141776149995123164	2020-09-18 02:26:01.224557	2020-09-18 02:26:01.224557
4438401999209499730	3984697827620770275	9141776149995123164	2020-09-18 02:26:01.224557	2020-09-18 02:26:01.224557
2148589948251146881	8791584697569536544	9141776149995123164	2020-09-18 02:26:01.224557	2020-09-18 02:26:01.224557
8227897328988947709	6750612841634615393	2448925535796042305	2020-09-18 02:28:14.428779	2020-09-18 02:28:14.428779
3693685575477736217	3984697827620770275	2448925535796042305	2020-09-18 02:28:14.428779	2020-09-18 02:28:14.428779
4318033600979881054	8791584697569536544	2448925535796042305	2020-09-18 02:28:14.428779	2020-09-18 02:28:14.428779
4584910345892533504	6750612841634615393	8352742620538922856	2020-09-18 03:29:54.102827	2020-09-18 03:29:54.102827
782312668647416747	3984697827620770275	8352742620538922856	2020-09-18 03:29:54.102827	2020-09-18 03:29:54.102827
660059441850105321	8791584697569536544	8352742620538922856	2020-09-18 03:29:54.102827	2020-09-18 03:29:54.102827
6752694556208096759	6750612841634615393	2838704115976202491	2020-09-08 08:16:11.916299	2020-09-08 08:16:11.916299
1608216632059556777	3984697827620770275	2838704115976202491	2020-09-08 08:16:11.916299	2020-09-08 08:16:11.916299
6728290598241209101	8791584697569536544	2838704115976202491	2020-09-08 08:16:11.916299	2020-09-08 08:16:11.916299
1637854268231126868	3667706950380863445	3455115140489977330	2020-09-08 08:51:32.408623	2020-09-08 08:51:32.408623
3137948147662715944	3534513442598030609	3455115140489977330	2020-09-08 08:51:33.842949	2020-09-08 08:51:33.842949
7633715617514679407	9075465603310523524	3455115140489977330	2020-09-08 08:51:36.391009	2020-09-08 08:51:36.391009
3142764328719262690	1862664629529357697	3455115140489977330	2020-09-08 08:51:37.206077	2020-09-08 08:51:37.206077
1988451045212357929	6505506293171167408	3455115140489977330	2020-09-08 08:51:38.622169	2020-09-08 08:51:38.622169
6326106101823543275	5488916325371064141	3455115140489977330	2020-09-08 08:51:39.751948	2020-09-08 08:51:39.751948
955438814911586931	6750612841634615393	276871354416890129	2020-09-08 09:13:25.012048	2020-09-08 09:13:25.012048
2583077313749385336	3984697827620770275	276871354416890129	2020-09-08 09:13:25.012048	2020-09-08 09:13:25.012048
8957155773923910528	8791584697569536544	276871354416890129	2020-09-08 09:13:25.012048	2020-09-08 09:13:25.012048
8542319598024218532	6750612841634615393	6219686592090773906	2020-09-08 11:22:50.18821	2020-09-08 11:22:50.18821
5715140029589706247	3984697827620770275	6219686592090773906	2020-09-08 11:22:50.18821	2020-09-08 11:22:50.18821
2481546468820123617	8791584697569536544	6219686592090773906	2020-09-08 11:22:50.18821	2020-09-08 11:22:50.18821
1448225190399662030	6750612841634615393	3170318566048626046	2020-09-09 01:49:31.055423	2020-09-09 01:49:31.055423
5456321801237772557	3984697827620770275	3170318566048626046	2020-09-09 01:49:31.055423	2020-09-09 01:49:31.055423
3736230662326222253	8791584697569536544	3170318566048626046	2020-09-09 01:49:31.055423	2020-09-09 01:49:31.055423
6493168791326141262	5488916325371064141	5601071935482074023	2020-09-09 02:30:50.709301	2020-09-09 02:30:50.709301
8543313723155374834	6505506293171167408	5601071935482074023	2020-09-09 02:30:51.501658	2020-09-09 02:30:51.501658
4900780416893868996	1862664629529357697	5601071935482074023	2020-09-09 02:30:53.334712	2020-09-09 02:30:53.334712
3785398767782063312	9075465603310523524	5601071935482074023	2020-09-09 02:30:54.510433	2020-09-09 02:30:54.510433
2286658945566547877	6750612841634615393	2302510692996666299	2020-09-09 11:26:35.436115	2020-09-09 11:26:35.436115
4938240346701452057	3984697827620770275	2302510692996666299	2020-09-09 11:26:35.436115	2020-09-09 11:26:35.436115
9207893875021594671	8791584697569536544	2302510692996666299	2020-09-09 11:26:35.436115	2020-09-09 11:26:35.436115
3723635764667206846	3667706950380863445	7659254709327742843	2020-09-10 03:22:37.691038	2020-09-10 03:22:37.691038
463312354721248550	3534513442598030609	7659254709327742843	2020-09-10 03:22:39.000464	2020-09-10 03:22:39.000464
4549577507411849744	6505506293171167408	7659254709327742843	2020-09-10 03:22:40.775319	2020-09-10 03:22:40.775319
7330943581898834495	5488916325371064141	7659254709327742843	2020-09-10 03:22:42.009117	2020-09-10 03:22:42.009117
836992977580628592	9075465603310523524	7659254709327742843	2020-09-10 03:22:43.801564	2020-09-10 03:22:43.801564
4751116804911796085	1862664629529357697	7659254709327742843	2020-09-10 03:22:45.027026	2020-09-10 03:22:45.027026
5212704030771255513	6750612841634615393	5252403087755855415	2020-09-10 03:33:55.038964	2020-09-10 03:33:55.038964
1792908045350171760	3984697827620770275	5252403087755855415	2020-09-10 03:33:55.038964	2020-09-10 03:33:55.038964
1681077334516217231	8791584697569536544	5252403087755855415	2020-09-10 03:33:55.038964	2020-09-10 03:33:55.038964
7491421100142428754	3667706950380863445	4305958976104284087	2020-09-10 06:15:19.365	2020-09-10 06:15:19.365
7287814572540416260	5488916325371064141	4305958976104284087	2020-09-10 06:15:21.417129	2020-09-10 06:15:21.417129
1574076639559723134	6750612841634615393	5986398665897825204	2020-09-10 09:50:55.497949	2020-09-10 09:50:55.497949
630605992173744667	3984697827620770275	5986398665897825204	2020-09-10 09:50:55.497949	2020-09-10 09:50:55.497949
7595902217729860173	8791584697569536544	5986398665897825204	2020-09-10 09:50:55.497949	2020-09-10 09:50:55.497949
4098383387305641084	6750612841634615393	5223911966781382373	2020-09-10 09:50:55.510754	2020-09-10 09:50:55.510754
5859030158197508774	3984697827620770275	5223911966781382373	2020-09-10 09:50:55.510754	2020-09-10 09:50:55.510754
7443951665351407718	8791584697569536544	5223911966781382373	2020-09-10 09:50:55.510754	2020-09-10 09:50:55.510754
3543363349283665050	6750612841634615393	8799391418637662520	2020-09-14 08:02:33.894112	2020-09-14 08:02:33.894112
8132888500911250372	3984697827620770275	8799391418637662520	2020-09-14 08:02:33.894112	2020-09-14 08:02:33.894112
7844382704973034003	8791584697569536544	8799391418637662520	2020-09-14 08:02:33.894112	2020-09-14 08:02:33.894112
2188969636445978812	3667706950380863445	7375716747774944843	2020-09-15 06:20:19.806483	2020-09-15 06:20:19.806483
3504682233824971471	3534513442598030609	7375716747774944843	2020-09-15 06:20:21.45764	2020-09-15 06:20:21.45764
7543430628805611582	6505506293171167408	7375716747774944843	2020-09-15 06:20:23.290681	2020-09-15 06:20:23.290681
7684783283080123118	5488916325371064141	7375716747774944843	2020-09-15 06:20:24.315733	2020-09-15 06:20:24.315733
5239878079157968873	9075465603310523524	7375716747774944843	2020-09-15 06:20:26.441015	2020-09-15 06:20:26.441015
3694770677916067467	1862664629529357697	7375716747774944843	2020-09-15 06:20:27.362208	2020-09-15 06:20:27.362208
8801479433161403178	6750612841634615393	8603229685798689003	2020-09-16 06:37:22.736904	2020-09-16 06:37:22.736904
5805220863055740014	3984697827620770275	8603229685798689003	2020-09-16 06:37:22.736904	2020-09-16 06:37:22.736904
232266769716010042	8791584697569536544	8603229685798689003	2020-09-16 06:37:22.736904	2020-09-16 06:37:22.736904
6919537011167704535	6750612841634615393	5694730814590643057	2020-09-17 06:41:40.144757	2020-09-17 06:41:40.144757
7323940993319469817	3984697827620770275	5694730814590643057	2020-09-17 06:41:40.144757	2020-09-17 06:41:40.144757
2185172376116079015	8791584697569536544	5694730814590643057	2020-09-17 06:41:40.144757	2020-09-17 06:41:40.144757
2404203827400699091	6750612841634615393	7037768559378452897	2020-09-18 02:12:44.939568	2020-09-18 02:12:44.939568
2342144205920833226	3984697827620770275	7037768559378452897	2020-09-18 02:12:44.939568	2020-09-18 02:12:44.939568
5424654153891917987	8791584697569536544	7037768559378452897	2020-09-18 02:12:44.939568	2020-09-18 02:12:44.939568
7375231104435798277	6750612841634615393	8699691129036934747	2020-09-18 03:00:53.024621	2020-09-18 03:00:53.024621
1172508508942790718	3984697827620770275	8699691129036934747	2020-09-18 03:00:53.024621	2020-09-18 03:00:53.024621
6309680979373348170	8791584697569536544	8699691129036934747	2020-09-18 03:00:53.024621	2020-09-18 03:00:53.024621
7274260653888045727	6750612841634615393	4365732954452568148	2020-09-08 08:39:57.243368	2020-09-08 08:39:57.243368
6907642494650092708	3984697827620770275	4365732954452568148	2020-09-08 08:39:57.243368	2020-09-08 08:39:57.243368
5400422126901889166	8791584697569536544	4365732954452568148	2020-09-08 08:39:57.243368	2020-09-08 08:39:57.243368
7012413891876299543	6750612841634615393	1775585148607179624	2020-09-08 08:55:49.537949	2020-09-08 08:55:49.537949
1433768200020897387	3984697827620770275	1775585148607179624	2020-09-08 08:55:49.537949	2020-09-08 08:55:49.537949
1490205822154793498	8791584697569536544	1775585148607179624	2020-09-08 08:55:49.537949	2020-09-08 08:55:49.537949
4479140892237093901	6750612841634615393	9069012371734241364	2020-09-08 08:55:49.559309	2020-09-08 08:55:49.559309
7592589690780729154	3984697827620770275	9069012371734241364	2020-09-08 08:55:49.559309	2020-09-08 08:55:49.559309
7168217813590796950	8791584697569536544	9069012371734241364	2020-09-08 08:55:49.559309	2020-09-08 08:55:49.559309
3465107127367664355	6750612841634615393	2598008582423324298	2020-09-08 08:55:49.565211	2020-09-08 08:55:49.565211
5162567811453653704	3984697827620770275	2598008582423324298	2020-09-08 08:55:49.565211	2020-09-08 08:55:49.565211
3356894080079074474	8791584697569536544	2598008582423324298	2020-09-08 08:55:49.565211	2020-09-08 08:55:49.565211
4197210734208671888	6750612841634615393	5394771594751493691	2020-09-08 08:55:49.570919	2020-09-08 08:55:49.570919
8668792595488509300	3984697827620770275	5394771594751493691	2020-09-08 08:55:49.570919	2020-09-08 08:55:49.570919
7848125590381592369	8791584697569536544	5394771594751493691	2020-09-08 08:55:49.570919	2020-09-08 08:55:49.570919
9172724142766383035	6750612841634615393	5353313573339177523	2020-09-08 08:55:49.575759	2020-09-08 08:55:49.575759
8471373834763721751	3984697827620770275	5353313573339177523	2020-09-08 08:55:49.575759	2020-09-08 08:55:49.575759
5523752600253659384	8791584697569536544	5353313573339177523	2020-09-08 08:55:49.575759	2020-09-08 08:55:49.575759
6765531774024845618	6750612841634615393	854473010740327354	2020-09-08 11:22:52.634961	2020-09-08 11:22:52.634961
1181570708746169868	3984697827620770275	854473010740327354	2020-09-08 11:22:52.634961	2020-09-08 11:22:52.634961
3822325277143021210	8791584697569536544	854473010740327354	2020-09-08 11:22:52.634961	2020-09-08 11:22:52.634961
101293907331711094	6750612841634615393	9065103658303460186	2020-09-09 01:49:33.886297	2020-09-09 01:49:33.886297
5489630630590871904	3984697827620770275	9065103658303460186	2020-09-09 01:49:33.886297	2020-09-09 01:49:33.886297
5540261081755473181	8791584697569536544	9065103658303460186	2020-09-09 01:49:33.886297	2020-09-09 01:49:33.886297
1735263029517958702	6750612841634615393	978896616369573409	2020-09-09 04:07:12.531873	2020-09-09 04:07:12.531873
1425067241422432020	3984697827620770275	978896616369573409	2020-09-09 04:07:12.531873	2020-09-09 04:07:12.531873
8132016664186181596	8791584697569536544	978896616369573409	2020-09-09 04:07:12.531873	2020-09-09 04:07:12.531873
831901392147323052	6750612841634615393	9055950420423069060	2020-09-09 11:27:22.847029	2020-09-09 11:27:22.847029
4435099181263167468	3984697827620770275	9055950420423069060	2020-09-09 11:27:22.847029	2020-09-09 11:27:22.847029
7734509120402243645	8791584697569536544	9055950420423069060	2020-09-09 11:27:22.847029	2020-09-09 11:27:22.847029
6030705167466823092	6750612841634615393	2612773216355420476	2020-09-10 03:30:17.017603	2020-09-10 03:30:17.017603
7084101429251341570	3984697827620770275	2612773216355420476	2020-09-10 03:30:17.017603	2020-09-10 03:30:17.017603
3702511058259498282	8791584697569536544	2612773216355420476	2020-09-10 03:30:17.017603	2020-09-10 03:30:17.017603
4505825606475001930	6505506293171167408	5252403087755855415	2020-09-10 03:34:07.194881	2020-09-10 03:34:07.194881
1401215897868858810	5488916325371064141	5252403087755855415	2020-09-10 03:34:08.079825	2020-09-10 03:34:08.079825
1864099159536295655	3534513442598030609	5252403087755855415	2020-09-10 03:34:36.926893	2020-09-10 03:34:36.926893
601488600054178310	3667706950380863445	5252403087755855415	2020-09-10 03:34:38.569742	2020-09-10 03:34:38.569742
2774169873703726401	9075465603310523524	5252403087755855415	2020-09-10 03:34:41.067205	2020-09-10 03:34:41.067205
1877264398876968795	1862664629529357697	5252403087755855415	2020-09-10 03:34:42.259638	2020-09-10 03:34:42.259638
1480729692604897692	6750612841634615393	2947423296548425905	2020-09-10 06:37:26.144455	2020-09-10 06:37:26.144455
5982505842349946156	3984697827620770275	2947423296548425905	2020-09-10 06:37:26.144455	2020-09-10 06:37:26.144455
8299966401119481082	8791584697569536544	2947423296548425905	2020-09-10 06:37:26.144455	2020-09-10 06:37:26.144455
898200964585810324	6750612841634615393	8587893535813273098	2020-09-10 10:03:13.623475	2020-09-10 10:03:13.623475
2344030099008415147	3984697827620770275	8587893535813273098	2020-09-10 10:03:13.623475	2020-09-10 10:03:13.623475
3417121234038417734	8791584697569536544	8587893535813273098	2020-09-10 10:03:13.623475	2020-09-10 10:03:13.623475
1015390283565841489	6750612841634615393	6575428629280265870	2020-09-10 10:03:13.631065	2020-09-10 10:03:13.631065
8417192579927181162	3984697827620770275	6575428629280265870	2020-09-10 10:03:13.631065	2020-09-10 10:03:13.631065
1910411181722783572	8791584697569536544	6575428629280265870	2020-09-10 10:03:13.631065	2020-09-10 10:03:13.631065
5688432001969903622	6750612841634615393	5429881535936798637	2020-09-14 10:20:24.277598	2020-09-14 10:20:24.277598
5173737636581871620	3984697827620770275	5429881535936798637	2020-09-14 10:20:24.277598	2020-09-14 10:20:24.277598
3077810953666306534	8791584697569536544	5429881535936798637	2020-09-14 10:20:24.277598	2020-09-14 10:20:24.277598
2077800706454975393	6750612841634615393	8281437102882339157	2020-09-15 09:04:28.988799	2020-09-15 09:04:28.988799
530508850904288837	3984697827620770275	8281437102882339157	2020-09-15 09:04:28.988799	2020-09-15 09:04:28.988799
7974726932354137132	8791584697569536544	8281437102882339157	2020-09-15 09:04:28.988799	2020-09-15 09:04:28.988799
6052919950132076108	6750612841634615393	681771573559560332	2020-09-17 02:33:28.126259	2020-09-17 02:33:28.126259
6420458468511294169	3984697827620770275	681771573559560332	2020-09-17 02:33:28.126259	2020-09-17 02:33:28.126259
3527074112267820405	8791584697569536544	681771573559560332	2020-09-17 02:33:28.126259	2020-09-17 02:33:28.126259
2759812622846647472	6750612841634615393	5918735720551767331	2020-09-17 06:42:22.332484	2020-09-17 06:42:22.332484
6748555012080458377	3984697827620770275	5918735720551767331	2020-09-17 06:42:22.332484	2020-09-17 06:42:22.332484
7319599115593152527	8791584697569536544	5918735720551767331	2020-09-17 06:42:22.332484	2020-09-17 06:42:22.332484
2912677844761378573	6750612841634615393	4177492899450748665	2020-09-18 02:18:40.628918	2020-09-18 02:18:40.628918
2939233733911404902	3984697827620770275	4177492899450748665	2020-09-18 02:18:40.628918	2020-09-18 02:18:40.628918
838995576840545138	8791584697569536544	4177492899450748665	2020-09-18 02:18:40.628918	2020-09-18 02:18:40.628918
8189916239208986303	6750612841634615393	8571848086878800496	2020-09-18 02:19:32.703148	2020-09-18 02:19:32.703148
474474474943394848	3984697827620770275	8571848086878800496	2020-09-18 02:19:32.703148	2020-09-18 02:19:32.703148
7597831695921138986	8791584697569536544	8571848086878800496	2020-09-18 02:19:32.703148	2020-09-18 02:19:32.703148
6747531229399318682	6750612841634615393	7530724731559347298	2020-09-18 02:20:51.260607	2020-09-18 02:20:51.260607
531935061227993437	3984697827620770275	7530724731559347298	2020-09-18 02:20:51.260607	2020-09-18 02:20:51.260607
1395778531898087654	8791584697569536544	7530724731559347298	2020-09-18 02:20:51.260607	2020-09-18 02:20:51.260607
7579640224254074326	6750612841634615393	4027022891871908826	2020-09-18 03:10:54.033975	2020-09-18 03:10:54.033975
7124649732968413839	3984697827620770275	4027022891871908826	2020-09-18 03:10:54.033975	2020-09-18 03:10:54.033975
2619231793527932182	8791584697569536544	4027022891871908826	2020-09-18 03:10:54.033975	2020-09-18 03:10:54.033975
7805441469280743713	6750612841634615393	5601071935482074023	2020-09-08 08:10:16.515446	2020-09-08 08:10:16.515446
4598834369008918104	3984697827620770275	5601071935482074023	2020-09-08 08:10:16.515446	2020-09-08 08:10:16.515446
2556387861496630534	8791584697569536544	5601071935482074023	2020-09-08 08:10:16.515446	2020-09-08 08:10:16.515446
582189256233449222	6750612841634615393	2714470504778644006	2020-09-08 08:46:37.975038	2020-09-08 08:46:37.975038
5908491129643348361	3984697827620770275	2714470504778644006	2020-09-08 08:46:37.975038	2020-09-08 08:46:37.975038
3206065794192764262	8791584697569536544	2714470504778644006	2020-09-08 08:46:37.975038	2020-09-08 08:46:37.975038
636681022148946607	6750612841634615393	3824108440689794940	2020-09-08 09:00:41.520728	2020-09-08 09:00:41.520728
3208565377976659552	3984697827620770275	3824108440689794940	2020-09-08 09:00:41.520728	2020-09-08 09:00:41.520728
2226758110059805732	8791584697569536544	3824108440689794940	2020-09-08 09:00:41.520728	2020-09-08 09:00:41.520728
1317672965535646719	6750612841634615393	6855857258826464301	2020-09-08 11:25:06.675688	2020-09-08 11:25:06.675688
5719846760985721489	3984697827620770275	6855857258826464301	2020-09-08 11:25:06.675688	2020-09-08 11:25:06.675688
3357871305141550901	8791584697569536544	6855857258826464301	2020-09-08 11:25:06.675688	2020-09-08 11:25:06.675688
288307336143424839	6750612841634615393	7731757567175278668	2020-09-08 11:25:10.193592	2020-09-08 11:25:10.193592
7477557376932595007	3984697827620770275	7731757567175278668	2020-09-08 11:25:10.193592	2020-09-08 11:25:10.193592
5696649240135777448	8791584697569536544	7731757567175278668	2020-09-08 11:25:10.193592	2020-09-08 11:25:10.193592
1189449164202998532	3667706950380863445	9065103658303460186	2020-09-09 01:56:57.795371	2020-09-09 01:56:57.795371
1252698382873330210	3534513442598030609	9065103658303460186	2020-09-09 01:57:00.666514	2020-09-09 01:57:00.666514
5092760953354022389	9075465603310523524	9065103658303460186	2020-09-09 01:57:05.716903	2020-09-09 01:57:05.716903
2192365258640843847	1862664629529357697	9065103658303460186	2020-09-09 01:57:21.344924	2020-09-09 01:57:21.344924
8147140590518522810	6505506293171167408	9065103658303460186	2020-09-09 01:57:23.648308	2020-09-09 01:57:23.648308
6671369976247076245	5488916325371064141	9065103658303460186	2020-09-09 01:57:24.968856	2020-09-09 01:57:24.968856
7560358701780162105	6750612841634615393	4202637381044109981	2020-09-09 04:14:39.317757	2020-09-09 04:14:39.317757
4346733657744370244	3984697827620770275	4202637381044109981	2020-09-09 04:14:39.317757	2020-09-09 04:14:39.317757
6397010510259254017	8791584697569536544	4202637381044109981	2020-09-09 04:14:39.317757	2020-09-09 04:14:39.317757
7037172625293034152	3667706950380863445	806107515131523577	2020-09-10 01:59:20.028018	2020-09-10 01:59:20.028018
6653936902682116167	3534513442598030609	806107515131523577	2020-09-10 01:59:23.680746	2020-09-10 01:59:23.680746
392978611399874240	6505506293171167408	806107515131523577	2020-09-10 01:59:26.498447	2020-09-10 01:59:26.498447
1721594599108794012	5488916325371064141	806107515131523577	2020-09-10 01:59:27.668166	2020-09-10 01:59:27.668166
8913984737525113792	9075465603310523524	806107515131523577	2020-09-10 01:59:29.714119	2020-09-10 01:59:29.714119
7213209902852636537	1862664629529357697	806107515131523577	2020-09-10 01:59:30.758092	2020-09-10 01:59:30.758092
4446147766891717533	3667706950380863445	2612773216355420476	2020-09-10 03:30:24.472931	2020-09-10 03:30:24.472931
2587231057537044760	3534513442598030609	2612773216355420476	2020-09-10 03:30:25.724098	2020-09-10 03:30:25.724098
3962872854603334511	5488916325371064141	2612773216355420476	2020-09-10 03:30:55.746894	2020-09-10 03:30:55.746894
2535077801764280400	6505506293171167408	2612773216355420476	2020-09-10 03:30:56.849071	2020-09-10 03:30:56.849071
7103332655754316475	9075465603310523524	2612773216355420476	2020-09-10 03:30:58.899769	2020-09-10 03:30:58.899769
2679483370840163772	1862664629529357697	2612773216355420476	2020-09-10 03:31:00.375611	2020-09-10 03:31:00.375611
6670946545953499690	6750612841634615393	7875265132999058905	2020-09-10 03:32:01.008575	2020-09-10 03:32:01.008575
721786795201862357	3984697827620770275	7875265132999058905	2020-09-10 03:32:01.008575	2020-09-10 03:32:01.008575
4843033417807215725	8791584697569536544	7875265132999058905	2020-09-10 03:32:01.008575	2020-09-10 03:32:01.008575
8953097509732038869	3534513442598030609	7875265132999058905	2020-09-10 03:32:10.12636	2020-09-10 03:32:10.12636
5734559435761004999	3667706950380863445	7875265132999058905	2020-09-10 03:32:11.068605	2020-09-10 03:32:11.068605
6161970507787202183	6750612841634615393	4305958976104284087	2020-09-10 06:12:07.621192	2020-09-10 06:12:07.621192
5737185130103341588	3984697827620770275	4305958976104284087	2020-09-10 06:12:07.621192	2020-09-10 06:12:07.621192
3439490190692413000	8791584697569536544	4305958976104284087	2020-09-10 06:12:07.621192	2020-09-10 06:12:07.621192
487621839375108654	3667706950380863445	2947423296548425905	2020-09-10 06:37:37.031928	2020-09-10 06:37:37.031928
5553588078473848548	3534513442598030609	2947423296548425905	2020-09-10 06:37:38.247785	2020-09-10 06:37:38.247785
391171450844138228	6505506293171167408	2947423296548425905	2020-09-10 06:37:41.645244	2020-09-10 06:37:41.645244
431805124156699902	5488916325371064141	2947423296548425905	2020-09-10 06:37:51.309289	2020-09-10 06:37:51.309289
2313827433052815872	9075465603310523524	2947423296548425905	2020-09-10 06:37:58.726307	2020-09-10 06:37:58.726307
2209529773650418820	1862664629529357697	2947423296548425905	2020-09-10 06:38:10.465603	2020-09-10 06:38:10.465603
527760364115218348	6750612841634615393	4959061375679117082	2020-09-10 10:51:28.617908	2020-09-10 10:51:28.617908
7418817415477160753	3984697827620770275	4959061375679117082	2020-09-10 10:51:28.617908	2020-09-10 10:51:28.617908
3992979731247470731	8791584697569536544	4959061375679117082	2020-09-10 10:51:28.617908	2020-09-10 10:51:28.617908
2478443948114541069	6750612841634615393	8147385450836295281	2020-09-10 10:51:28.639593	2020-09-10 10:51:28.639593
6663611090948457454	3984697827620770275	8147385450836295281	2020-09-10 10:51:28.639593	2020-09-10 10:51:28.639593
5729944592277237468	8791584697569536544	8147385450836295281	2020-09-10 10:51:28.639593	2020-09-10 10:51:28.639593
1381030034577526304	6750612841634615393	188137472857428845	2020-09-14 11:24:16.348836	2020-09-14 11:24:16.348836
8409438311034600005	3984697827620770275	188137472857428845	2020-09-14 11:24:16.348836	2020-09-14 11:24:16.348836
8932272000018239688	8791584697569536544	188137472857428845	2020-09-14 11:24:16.348836	2020-09-14 11:24:16.348836
1344675711124777718	6750612841634615393	6457364932904658384	2020-09-15 10:41:04.073455	2020-09-15 10:41:04.073455
2219003953322195292	3984697827620770275	6457364932904658384	2020-09-15 10:41:04.073455	2020-09-15 10:41:04.073455
2522612787236669868	8791584697569536544	6457364932904658384	2020-09-15 10:41:04.073455	2020-09-15 10:41:04.073455
1504828078483943053	6750612841634615393	2138533638807468853	2020-09-17 06:10:37.008079	2020-09-17 06:10:37.008079
898246692986084940	3984697827620770275	2138533638807468853	2020-09-17 06:10:37.008079	2020-09-17 06:10:37.008079
7510280444607607142	8791584697569536544	2138533638807468853	2020-09-17 06:10:37.008079	2020-09-17 06:10:37.008079
1866117621767508939	6750612841634615393	3871910907348016407	2020-09-17 07:32:16.61978	2020-09-17 07:32:16.61978
7430335547048065169	3984697827620770275	3871910907348016407	2020-09-17 07:32:16.61978	2020-09-17 07:32:16.61978
7846965445158664380	8791584697569536544	3871910907348016407	2020-09-17 07:32:16.61978	2020-09-17 07:32:16.61978
8911667284996438489	6750612841634615393	8565050772142024576	2020-09-18 02:22:53.309382	2020-09-18 02:22:53.309382
5474149781460277276	3984697827620770275	8565050772142024576	2020-09-18 02:22:53.309382	2020-09-18 02:22:53.309382
2444655897629917604	8791584697569536544	8565050772142024576	2020-09-18 02:22:53.309382	2020-09-18 02:22:53.309382
7973320620294017409	6750612841634615393	9037228755693796387	2020-09-18 03:16:58.748084	2020-09-18 03:16:58.748084
6639710432023962254	3984697827620770275	9037228755693796387	2020-09-18 03:16:58.748084	2020-09-18 03:16:58.748084
4342581551185535927	8791584697569536544	9037228755693796387	2020-09-18 03:16:58.748084	2020-09-18 03:16:58.748084
1768521379813149564	6750612841634615393	3707743263319537005	2020-09-08 08:11:39.262375	2020-09-08 08:11:39.262375
5213672013304451371	3984697827620770275	3707743263319537005	2020-09-08 08:11:39.262375	2020-09-08 08:11:39.262375
7474112539724200069	8791584697569536544	3707743263319537005	2020-09-08 08:11:39.262375	2020-09-08 08:11:39.262375
6650367813404765427	6750612841634615393	3455115140489977330	2020-09-08 08:50:26.407567	2020-09-08 08:50:26.407567
3642608981984088671	3984697827620770275	3455115140489977330	2020-09-08 08:50:26.407567	2020-09-08 08:50:26.407567
7104257450520501150	8791584697569536544	3455115140489977330	2020-09-08 08:50:26.407567	2020-09-08 08:50:26.407567
\.


--
-- Data for Name: user_player_tracks; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.user_player_tracks (tid, pid, uid, rotaion_x, rotaion_y, rotaion_z, location_x, location_y, location_z, modify_time, created_time) FROM stdin;
\.


--
-- Data for Name: user_players; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.user_players (id, pid, uid, max_hp, attack_power, move_speed, max_mana, defense, level, star_level, level_experience, is_default, modify_time, created_time) FROM stdin;
5625161856560138982	900238616481714883	2318448884852593867	20	5	600	20	5	1	1	120	2	2020-09-08 11:08:52.042273	2020-09-08 11:08:52.042273
3002783530245768144	900238616481714883	6219686592090773906	20	5	600	20	5	1	1	120	2	2020-09-08 11:22:50.18821	2020-09-08 11:22:50.18821
5644856964542308236	900238616481714883	854473010740327354	20	5	600	20	5	1	1	120	2	2020-09-08 11:22:52.634961	2020-09-08 11:22:52.634961
5242786136012121978	900238616481714883	6855857258826464301	20	5	600	20	5	1	1	120	2	2020-09-08 11:25:06.675688	2020-09-08 11:25:06.675688
5709982286360068273	900238616481714883	7731757567175278668	20	5	600	20	5	1	1	120	2	2020-09-08 11:25:10.193592	2020-09-08 11:25:10.193592
8789199977831714274	900238616481714883	3694901305134076376	20	5	600	20	5	1	1	120	2	2020-09-08 11:41:41.765753	2020-09-08 11:41:41.765753
1627212740272933324	900238616481714883	1193295810120197710	20	5	600	20	5	1	1	120	2	2020-09-08 11:41:44.575887	2020-09-08 11:41:44.575887
7420077525030550318	900238616481714883	3170318566048626046	20	5	600	20	5	1	1	120	2	2020-09-09 01:49:31.055423	2020-09-09 01:49:31.055423
4186019827493195344	900238616481714883	9065103658303460186	20	5	600	20	5	1	1	120	2	2020-09-09 01:49:33.886297	2020-09-09 01:49:33.886297
7675640595331657324	900238616481714883	978896616369573409	20	5	600	20	5	1	1	120	2	2020-09-09 04:07:12.531873	2020-09-09 04:07:12.531873
1602485277609787155	900238616481714883	4202637381044109981	20	5	600	20	5	1	1	120	2	2020-09-09 04:14:39.317757	2020-09-09 04:14:39.317757
535068797139300126	900238616481714883	806107515131523577	20	5	600	20	5	1	1	120	2	2020-09-09 06:02:41.179819	2020-09-09 06:02:41.179819
4472025296784888735	900238616481714883	2302510692996666299	20	5	600	20	5	1	1	120	2	2020-09-09 11:26:35.436115	2020-09-09 11:26:35.436115
1927006054953625903	900238616481714883	9055950420423069060	20	5	600	20	5	1	1	120	2	2020-09-09 11:27:22.847029	2020-09-09 11:27:22.847029
4443693152937372201	900238616481714883	7659254709327742843	20	5	600	20	5	1	1	120	2	2020-09-10 03:22:08.368096	2020-09-10 03:22:08.368096
7903068537609550740	900238616481714883	2612773216355420476	20	5	600	20	5	1	1	120	2	2020-09-10 03:30:17.017603	2020-09-10 03:30:17.017603
3587502090642909454	900238616481714883	7875265132999058905	20	5	600	20	5	1	1	120	2	2020-09-10 03:32:01.008575	2020-09-10 03:32:01.008575
7593199315742830275	900238616481714883	5252403087755855415	20	5	600	20	5	1	1	120	2	2020-09-10 03:33:55.038964	2020-09-10 03:33:55.038964
4613794039240444344	900238616481714883	4305958976104284087	20	5	600	20	5	1	1	120	2	2020-09-10 06:12:07.621192	2020-09-10 06:12:07.621192
2096077798075058212	900238616481714883	2947423296548425905	20	5	600	20	5	1	1	120	2	2020-09-10 06:37:26.144455	2020-09-10 06:37:26.144455
2951101842368853378	900238616481714883	4833729286789198324	20	5	600	20	5	1	1	120	2	2020-09-10 07:17:11.881891	2020-09-10 07:17:11.881891
5556382404785043194	900238616481714883	5986398665897825204	20	5	600	20	5	1	1	120	2	2020-09-10 09:50:55.497949	2020-09-10 09:50:55.497949
3990061157706022094	900238616481714883	5223911966781382373	20	5	600	20	5	1	1	120	2	2020-09-10 09:50:55.510754	2020-09-10 09:50:55.510754
6293526094870192183	900238616481714883	8587893535813273098	20	5	600	20	5	1	1	120	2	2020-09-10 10:03:13.623475	2020-09-10 10:03:13.623475
6640434150896235023	900238616481714883	6575428629280265870	20	5	600	20	5	1	1	120	2	2020-09-10 10:03:13.631065	2020-09-10 10:03:13.631065
7331824446430101542	900238616481714883	4959061375679117082	20	5	600	20	5	1	1	120	2	2020-09-10 10:51:28.617908	2020-09-10 10:51:28.617908
8099324897135120804	900238616481714883	8147385450836295281	20	5	600	20	5	1	1	120	2	2020-09-10 10:51:28.639593	2020-09-10 10:51:28.639593
2621802823719505285	900238616481714883	1041955479368729000	20	5	600	20	5	1	1	120	2	2020-09-11 01:50:36.251302	2020-09-11 01:50:36.251302
8396616679518524921	900238616481714883	1981557320191779142	20	5	600	20	5	1	1	120	2	2020-09-11 01:50:36.266492	2020-09-11 01:50:36.266492
950776482796669857	900238616481714883	8799391418637662520	20	5	600	20	5	1	1	120	2	2020-09-14 08:02:33.894112	2020-09-14 08:02:33.894112
8695082920643902052	900238616481714883	5429881535936798637	20	5	600	20	5	1	1	120	2	2020-09-14 10:20:24.277598	2020-09-14 10:20:24.277598
7396370259058064897	900238616481714883	188137472857428845	20	5	600	20	5	1	1	120	2	2020-09-14 11:24:16.348836	2020-09-14 11:24:16.348836
8684972459767727204	900238616481714883	7375716747774944843	20	5	600	20	5	1	1	120	2	2020-09-15 06:12:38.63696	2020-09-15 06:12:38.63696
2105701729229834881	900238616481714883	8281437102882339157	20	5	600	20	5	1	1	120	2	2020-09-15 09:04:28.988799	2020-09-15 09:04:28.988799
4110907379946245558	900238616481714883	6457364932904658384	20	5	600	20	5	1	1	120	2	2020-09-15 10:41:04.073455	2020-09-15 10:41:04.073455
1341335025128524399	900238616481714883	2694133533970932102	20	5	600	20	5	1	1	120	2	2020-09-16 02:43:25.246742	2020-09-16 02:43:25.246742
6122860821799821209	900238616481714883	8603229685798689003	20	5	600	20	5	1	1	120	2	2020-09-16 06:37:22.736904	2020-09-16 06:37:22.736904
9025410734071981547	900238616481714883	681771573559560332	20	5	600	20	5	1	1	120	2	2020-09-17 02:33:28.126259	2020-09-17 02:33:28.126259
8076237013698095043	900238616481714883	2138533638807468853	20	5	600	20	5	1	1	120	2	2020-09-17 06:10:37.008079	2020-09-17 06:10:37.008079
4595473553582085355	900238616481714883	7952244182381372668	20	5	600	20	5	1	1	120	2	2020-09-17 06:38:53.265833	2020-09-17 06:38:53.265833
7595089223104956403	900238616481714883	5694730814590643057	20	5	600	20	5	1	1	120	2	2020-09-17 06:41:40.144757	2020-09-17 06:41:40.144757
4087214202485332345	900238616481714883	5918735720551767331	20	5	600	20	5	1	1	120	2	2020-09-17 06:42:22.332484	2020-09-17 06:42:22.332484
2163894271505751652	900238616481714883	3871910907348016407	20	5	600	20	5	1	1	120	2	2020-09-17 07:32:16.61978	2020-09-17 07:32:16.61978
1207429156642431845	900238616481714883	7841791345558273926	20	5	600	20	5	1	1	120	2	2020-09-17 10:56:10.108799	2020-09-17 10:56:10.108799
6368557963914194524	900238616481714883	7037768559378452897	20	5	600	20	5	1	1	120	2	2020-09-18 02:12:44.939568	2020-09-18 02:12:44.939568
476125990885239694	900238616481714883	4177492899450748665	20	5	600	20	5	1	1	120	2	2020-09-18 02:18:40.628918	2020-09-18 02:18:40.628918
4491726056881263874	900238616481714883	8571848086878800496	20	5	600	20	5	1	1	120	2	2020-09-18 02:19:32.703148	2020-09-18 02:19:32.703148
7142959999897457747	900238616481714883	7530724731559347298	20	5	600	20	5	1	1	120	2	2020-09-18 02:20:51.260607	2020-09-18 02:20:51.260607
3231915002073911357	900238616481714883	8565050772142024576	20	5	600	20	5	1	1	120	2	2020-09-18 02:22:53.309382	2020-09-18 02:22:53.309382
1236630269510402693	900238616481714883	9141776149995123164	20	5	600	20	5	1	1	120	2	2020-09-18 02:26:01.224557	2020-09-18 02:26:01.224557
202293538665993405	900238616481714883	2448925535796042305	20	5	600	20	5	1	1	120	2	2020-09-18 02:28:14.428779	2020-09-18 02:28:14.428779
3697968101983211661	900238616481714883	5601071935482074023	20	5	600	20	5	1	1	120	2	2020-09-08 08:10:16.515446	2020-09-08 08:10:16.515446
2259337918305388393	900238616481714883	3707743263319537005	20	5	600	20	5	1	1	120	2	2020-09-08 08:11:39.262375	2020-09-08 08:11:39.262375
5378723711617832341	900238616481714883	2838704115976202491	20	5	600	20	5	1	1	120	2	2020-09-08 08:16:11.916299	2020-09-08 08:16:11.916299
92714939128066473	900238616481714883	4365732954452568148	20	5	600	20	5	1	1	120	2	2020-09-08 08:39:57.243368	2020-09-08 08:39:57.243368
367389436900242527	900238616481714883	2714470504778644006	20	5	600	20	5	1	1	120	2	2020-09-08 08:46:37.975038	2020-09-08 08:46:37.975038
5495210510752964933	900238616481714883	3455115140489977330	20	5	600	20	5	1	1	120	2	2020-09-08 08:50:26.407567	2020-09-08 08:50:26.407567
8686637546421294936	900238616481714883	1775585148607179624	20	5	600	20	5	1	1	120	2	2020-09-08 08:55:49.537949	2020-09-08 08:55:49.537949
4199040508838029002	900238616481714883	9069012371734241364	20	5	600	20	5	1	1	120	2	2020-09-08 08:55:49.559309	2020-09-08 08:55:49.559309
5699415959952721075	900238616481714883	2598008582423324298	20	5	600	20	5	1	1	120	2	2020-09-08 08:55:49.565211	2020-09-08 08:55:49.565211
6078447007247478966	900238616481714883	5394771594751493691	20	5	600	20	5	1	1	120	2	2020-09-08 08:55:49.570919	2020-09-08 08:55:49.570919
5171005275719211593	900238616481714883	5353313573339177523	20	5	600	20	5	1	1	120	2	2020-09-08 08:55:49.575759	2020-09-08 08:55:49.575759
3724600790749949871	900238616481714883	3824108440689794940	20	5	600	20	5	1	1	120	2	2020-09-08 09:00:41.520728	2020-09-08 09:00:41.520728
8530212335209228341	900238616481714883	5271470258722089835	20	5	600	20	5	1	1	120	2	2020-09-08 09:09:00.201487	2020-09-08 09:09:00.201487
46412439028413849	900238616481714883	6309855246227066278	20	5	600	20	5	1	1	120	2	2020-09-08 09:09:00.223674	2020-09-08 09:09:00.223674
4017345739243826618	900238616481714883	8326186895181328012	20	5	600	20	5	1	1	120	2	2020-09-08 09:09:00.229257	2020-09-08 09:09:00.229257
8321796053649735735	900238616481714883	6627591757152218867	20	5	600	20	5	1	1	120	2	2020-09-08 09:09:00.234443	2020-09-08 09:09:00.234443
277773475749919066	900238616481714883	329739458029569425	20	5	600	20	5	1	1	120	2	2020-09-08 09:09:00.237709	2020-09-08 09:09:00.237709
2860239641394832512	900238616481714883	276871354416890129	20	5	600	20	5	1	1	120	2	2020-09-08 09:13:25.012048	2020-09-08 09:13:25.012048
5967942694555325564	900238616481714883	8699691129036934747	20	5	600	20	5	1	1	120	2	2020-09-18 03:00:53.024621	2020-09-18 03:00:53.024621
8946070566183886208	900238616481714883	4027022891871908826	20	5	600	20	5	1	1	120	2	2020-09-18 03:10:54.033975	2020-09-18 03:10:54.033975
3435723005730782092	900238616481714883	9037228755693796387	20	5	600	20	5	1	1	120	2	2020-09-18 03:16:58.748084	2020-09-18 03:16:58.748084
5575202507324579902	900238616481714883	8352742620538922856	20	5	600	20	5	1	1	120	2	2020-09-18 03:29:54.102827	2020-09-18 03:29:54.102827
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: tfaadmin
--

COPY public.users (uuid, uid, name, avatar, login_days, server_id, modify_time, created_time, action_force) FROM stdin;
5601071935482074023	10006507	Governor10006507	game://2	1	1001	2020-09-08 08:10:16.515446	2020-09-08 08:10:16.515446	1000
3707743263319537005	10006508	Governor10006508	game://2	1	1001	2020-09-08 08:11:39.262375	2020-09-08 08:11:39.262375	1000
2838704115976202491	10006509	Governor10006509	game://2	1	1001	2020-09-08 08:16:11.916299	2020-09-08 08:16:11.916299	1000
4365732954452568148	10006510	Governor10006510	game://2	1	1001	2020-09-08 08:39:57.243368	2020-09-08 08:39:57.243368	1000
2714470504778644006	10006511	Governor10006511	game://1	1	1001	2020-09-08 08:46:37.975038	2020-09-08 08:46:37.975038	1000
3455115140489977330	10006512	我比较特别	game://1	1	1001	2020-09-08 08:50:26.407567	2020-09-08 08:50:26.407567	1000
1775585148607179624	10006513	Governor10006513	game://1	1	1001	2020-09-08 08:55:49.537949	2020-09-08 08:55:49.537949	1000
9069012371734241364	10006514	Governor10006514	game://2	1	1001	2020-09-08 08:55:49.559309	2020-09-08 08:55:49.559309	1000
2598008582423324298	10006515	Governor10006515	game://2	1	1001	2020-09-08 08:55:49.565211	2020-09-08 08:55:49.565211	1000
5394771594751493691	10006516	Governor10006516	game://2	1	1001	2020-09-08 08:55:49.570919	2020-09-08 08:55:49.570919	1000
5353313573339177523	10006517	Governor10006517	game://1	1	1001	2020-09-08 08:55:49.575759	2020-09-08 08:55:49.575759	1000
3824108440689794940	10006518	Governor10006518	game://1	1	1001	2020-09-08 09:00:41.520728	2020-09-08 09:00:41.520728	1000
5271470258722089835	10006519	Governor10006519	game://2	1	1001	2020-09-08 09:09:00.201487	2020-09-08 09:09:00.201487	1000
6309855246227066278	10006520	Governor10006520	game://2	1	1001	2020-09-08 09:09:00.223674	2020-09-08 09:09:00.223674	1000
8326186895181328012	10006521	Governor10006521	game://2	1	1001	2020-09-08 09:09:00.229257	2020-09-08 09:09:00.229257	1000
6627591757152218867	10006522	Governor10006522	game://1	1	1001	2020-09-08 09:09:00.234443	2020-09-08 09:09:00.234443	1000
329739458029569425	10006523	Governor10006523	game://1	1	1001	2020-09-08 09:09:00.237709	2020-09-08 09:09:00.237709	1000
276871354416890129	10006524	Governor10006524	game://1	1	1001	2020-09-08 09:13:25.012048	2020-09-08 09:13:25.012048	1000
2318448884852593867	10006525	Governor10006525	game://2	1	1001	2020-09-08 11:08:52.042273	2020-09-08 11:08:52.042273	1000
6219686592090773906	10006526	Governor10006526	game://1	1	1001	2020-09-08 11:22:50.18821	2020-09-08 11:22:50.18821	1000
854473010740327354	10006527	Governor10006527	game://2	1	1001	2020-09-08 11:22:52.634961	2020-09-08 11:22:52.634961	1000
6855857258826464301	10006528	Governor10006528	game://1	1	1001	2020-09-08 11:25:06.675688	2020-09-08 11:25:06.675688	1000
7731757567175278668	10006529	Governor10006529	game://1	1	1001	2020-09-08 11:25:10.193592	2020-09-08 11:25:10.193592	1000
3694901305134076376	10006530	Governor10006530	game://1	1	1001	2020-09-08 11:41:41.765753	2020-09-08 11:41:41.765753	1000
1193295810120197710	10006531	Governor10006531	game://2	1	1001	2020-09-08 11:41:44.575887	2020-09-08 11:41:44.575887	1000
3170318566048626046	10006532	Governor10006532	game://2	1	1001	2020-09-09 01:49:31.055423	2020-09-09 01:49:31.055423	1000
9065103658303460186	10006533	Governor10006533	game://2	1	1001	2020-09-09 01:49:33.886297	2020-09-09 01:49:33.886297	1000
978896616369573409	10006534	Governor10006534	game://1	1	1001	2020-09-09 04:07:12.531873	2020-09-09 04:07:12.531873	1000
4202637381044109981	10006535	Governor10006535	game://1	1	1001	2020-09-09 04:14:39.317757	2020-09-09 04:14:39.317757	1000
806107515131523577	10006536	Governor10006536	game://1	1	1001	2020-09-09 06:02:41.179819	2020-09-09 06:02:41.179819	1000
2302510692996666299	10006537	Governor10006537	game://2	1	1001	2020-09-09 11:26:35.436115	2020-09-09 11:26:35.436115	1000
9055950420423069060	10006538	Governor10006538	game://1	1	1001	2020-09-09 11:27:22.847029	2020-09-09 11:27:22.847029	1000
7659254709327742843	10006539	Governor10006539	game://1	1	1001	2020-09-10 03:22:08.368096	2020-09-10 03:22:08.368096	1000
2612773216355420476	10006540	Governor10006540	game://2	1	1001	2020-09-10 03:30:17.017603	2020-09-10 03:30:17.017603	1000
7875265132999058905	10006541	Governor10006541	game://1	1	1001	2020-09-10 03:32:01.008575	2020-09-10 03:32:01.008575	1000
5252403087755855415	10006542	Governor10006542	game://1	1	1001	2020-09-10 03:33:55.038964	2020-09-10 03:33:55.038964	1000
4305958976104284087	10006543	Governor10006543	game://2	1	1001	2020-09-10 06:12:07.621192	2020-09-10 06:12:07.621192	1000
2947423296548425905	10006544	Governor10006544	game://1	1	1001	2020-09-10 06:37:26.144455	2020-09-10 06:37:26.144455	1000
4833729286789198324	10006545	Governor10006545	game://2	1	1001	2020-09-10 07:17:11.881891	2020-09-10 07:17:11.881891	1000
5986398665897825204	10006546	Governor10006546	game://2	1	1001	2020-09-10 09:50:55.497949	2020-09-10 09:50:55.497949	1000
5223911966781382373	10006547	Governor10006547	game://1	1	1001	2020-09-10 09:50:55.510754	2020-09-10 09:50:55.510754	1000
8587893535813273098	10006548	Governor10006548	game://2	1	1001	2020-09-10 10:03:13.623475	2020-09-10 10:03:13.623475	1000
6575428629280265870	10006549	Governor10006549	game://2	1	1001	2020-09-10 10:03:13.631065	2020-09-10 10:03:13.631065	1000
4959061375679117082	10006550	Governor10006550	game://1	1	1001	2020-09-10 10:51:28.617908	2020-09-10 10:51:28.617908	1000
8147385450836295281	10006551	Governor10006551	game://2	1	1001	2020-09-10 10:51:28.639593	2020-09-10 10:51:28.639593	1000
1041955479368729000	10006552	Governor10006552	game://2	1	1001	2020-09-11 01:50:36.251302	2020-09-11 01:50:36.251302	1000
1981557320191779142	10006553	Governor10006553	game://2	1	1001	2020-09-11 01:50:36.266492	2020-09-11 01:50:36.266492	1000
8799391418637662520	10006554	Governor10006554	game://1	1	1001	2020-09-14 08:02:33.894112	2020-09-14 08:02:33.894112	1000
5429881535936798637	10006555	Governor10006555	game://1	1	1001	2020-09-14 10:20:24.277598	2020-09-14 10:20:24.277598	1000
188137472857428845	10006556	Governor10006556	game://2	1	1001	2020-09-14 11:24:16.348836	2020-09-14 11:24:16.348836	1000
7375716747774944843	10006557	Governor10006557	game://2	1	1001	2020-09-15 06:12:38.63696	2020-09-15 06:12:38.63696	1000
8281437102882339157	10006558	Governor10006558	game://2	1	1001	2020-09-15 09:04:28.988799	2020-09-15 09:04:28.988799	1000
6457364932904658384	10006559	Governor10006559	game://2	1	1001	2020-09-15 10:41:04.073455	2020-09-15 10:41:04.073455	1000
2694133533970932102	10006560	Governor10006560	game://1	1	1001	2020-09-16 02:43:25.246742	2020-09-16 02:43:25.246742	1000
8603229685798689003	10006561	Governor10006561	game://2	1	1001	2020-09-16 06:37:22.736904	2020-09-16 06:37:22.736904	1000
681771573559560332	10006562	Governor10006562	game://1	1	1001	2020-09-17 02:33:28.126259	2020-09-17 02:33:28.126259	1000
2138533638807468853	10006563	Governor10006563	game://1	1	1001	2020-09-17 06:10:37.008079	2020-09-17 06:10:37.008079	1000
7952244182381372668	10006564	Governor10006564	game://1	1	1001	2020-09-17 06:38:53.265833	2020-09-17 06:38:53.265833	1000
5694730814590643057	10006565	Governor10006565	game://1	1	1001	2020-09-17 06:41:40.144757	2020-09-17 06:41:40.144757	1000
5918735720551767331	10006566	Governor10006566	game://2	1	1001	2020-09-17 06:42:22.332484	2020-09-17 06:42:22.332484	1000
3871910907348016407	10006567	Governor10006567	game://2	1	1001	2020-09-17 07:32:16.61978	2020-09-17 07:32:16.61978	1000
7841791345558273926	10006568	Governor10006568	game://1	1	1001	2020-09-17 10:56:10.108799	2020-09-17 10:56:10.108799	1000
7037768559378452897	10006569	Governor10006569	game://2	1	1001	2020-09-18 02:12:44.939568	2020-09-18 02:12:44.939568	1000
4177492899450748665	10006570	Governor10006570	game://1	1	1001	2020-09-18 02:18:40.628918	2020-09-18 02:18:40.628918	1000
8571848086878800496	10006571	Governor10006571	game://1	1	1001	2020-09-18 02:19:32.703148	2020-09-18 02:19:32.703148	1000
7530724731559347298	10006572	Governor10006572	game://1	1	1001	2020-09-18 02:20:51.260607	2020-09-18 02:20:51.260607	1000
8565050772142024576	10006573	Governor10006573	game://1	1	1001	2020-09-18 02:22:53.309382	2020-09-18 02:22:53.309382	1000
9141776149995123164	10006574	Governor10006574	game://2	1	1001	2020-09-18 02:26:01.224557	2020-09-18 02:26:01.224557	1000
2448925535796042305	10006575	Governor10006575	game://2	1	1001	2020-09-18 02:28:14.428779	2020-09-18 02:28:14.428779	1000
8699691129036934747	10006576	Governor10006576	game://1	1	1001	2020-09-18 03:00:53.024621	2020-09-18 03:00:53.024621	1000
4027022891871908826	10006577	Governor10006577	game://2	1	1001	2020-09-18 03:10:54.033975	2020-09-18 03:10:54.033975	1000
9037228755693796387	10006578	Governor10006578	game://2	1	1001	2020-09-18 03:16:58.748084	2020-09-18 03:16:58.748084	1000
8352742620538922856	10006579	Governor10006579	game://2	1	1001	2020-09-18 03:29:54.102827	2020-09-18 03:29:54.102827	1000
\.


--
-- Name: users_login_days_seq; Type: SEQUENCE SET; Schema: public; Owner: tfaadmin
--

SELECT pg_catalog.setval('public.users_login_days_seq', 1, false);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: blacklists blacklists_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.blacklists
    ADD CONSTRAINT blacklists_pkey PRIMARY KEY (bid);


--
-- Name: chat_groups chat_groups_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.chat_groups
    ADD CONSTRAINT chat_groups_pkey PRIMARY KEY (gid);


--
-- Name: chat_groups_uids chat_groups_uids_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.chat_groups_uids
    ADD CONSTRAINT chat_groups_uids_pkey PRIMARY KEY (guid);


--
-- Name: chat_messages chat_messages_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.chat_messages
    ADD CONSTRAINT chat_messages_pkey PRIMARY KEY (mid);


--
-- Name: enemys enemys_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.enemys
    ADD CONSTRAINT enemys_pkey PRIMARY KEY (eid);


--
-- Name: equipment_kinds equipment_kinds_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.equipment_kinds
    ADD CONSTRAINT equipment_kinds_pkey PRIMARY KEY (kid);


--
-- Name: equipments equipments_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.equipments
    ADD CONSTRAINT equipments_pkey PRIMARY KEY (eid);


--
-- Name: friends friends_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.friends
    ADD CONSTRAINT friends_pkey PRIMARY KEY (fid);


--
-- Name: gem_relateds gem_relateds_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.gem_relateds
    ADD CONSTRAINT gem_relateds_pkey PRIMARY KEY (grid);


--
-- Name: gems gems_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.gems
    ADD CONSTRAINT gems_pkey PRIMARY KEY (gid);


--
-- Name: link_accounts link_accounts_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.link_accounts
    ADD CONSTRAINT link_accounts_pkey PRIMARY KEY (lid);


--
-- Name: mall_first_recharge_gift_package_assets mall_first_recharge_gift_package_assets_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.mall_first_recharge_gift_package_assets
    ADD CONSTRAINT mall_first_recharge_gift_package_assets_pkey PRIMARY KEY (id);


--
-- Name: mall_first_recharge_gift_packages mall_first_recharge_gift_packages_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.mall_first_recharge_gift_packages
    ADD CONSTRAINT mall_first_recharge_gift_packages_pkey PRIMARY KEY (id);


--
-- Name: mall_gem_stores mall_gem_stores_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.mall_gem_stores
    ADD CONSTRAINT mall_gem_stores_pkey PRIMARY KEY (id);


--
-- Name: mall_supply_station_assets mall_supply_station_assets_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.mall_supply_station_assets
    ADD CONSTRAINT mall_supply_station_assets_pkey PRIMARY KEY (id);


--
-- Name: mall_supply_stations mall_supply_stations_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.mall_supply_stations
    ADD CONSTRAINT mall_supply_stations_pkey PRIMARY KEY (id);


--
-- Name: mall_user_buy_gem_store_records mall_user_buy_gem_store_records_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.mall_user_buy_gem_store_records
    ADD CONSTRAINT mall_user_buy_gem_store_records_pkey PRIMARY KEY (rid);


--
-- Name: mall_user_buy_supply_station_records mall_user_buy_supply_station_records_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.mall_user_buy_supply_station_records
    ADD CONSTRAINT mall_user_buy_supply_station_records_pkey PRIMARY KEY (id);


--
-- Name: mall_user_first_recharge_gift_package_records mall_user_first_recharge_gift_package_records_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.mall_user_first_recharge_gift_package_records
    ADD CONSTRAINT mall_user_first_recharge_gift_package_records_pkey PRIMARY KEY (id);


--
-- Name: player_mount_equipments player_mount_equipments_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.player_mount_equipments
    ADD CONSTRAINT player_mount_equipments_pkey PRIMARY KEY (id);


--
-- Name: players players_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.players
    ADD CONSTRAINT players_pkey PRIMARY KEY (pid);


--
-- Name: props_resources_categorys props_resources_categorys_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.props_resources_categorys
    ADD CONSTRAINT props_resources_categorys_pkey PRIMARY KEY (rid);


--
-- Name: props_speed_up_categorys props_speed_up_categorys_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.props_speed_up_categorys
    ADD CONSTRAINT props_speed_up_categorys_pkey PRIMARY KEY (sid);


--
-- Name: purchase_orders purchase_orders_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.purchase_orders
    ADD CONSTRAINT purchase_orders_pkey PRIMARY KEY (oid);


--
-- Name: server_lists server_lists_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.server_lists
    ADD CONSTRAINT server_lists_pkey PRIMARY KEY (slid);


--
-- Name: servers servers_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.servers
    ADD CONSTRAINT servers_pkey PRIMARY KEY (sid);


--
-- Name: skill_fight_relateds skill_fight_relateds_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.skill_fight_relateds
    ADD CONSTRAINT skill_fight_relateds_pkey PRIMARY KEY (id);


--
-- Name: skills skills_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.skills
    ADD CONSTRAINT skills_pkey PRIMARY KEY (id);


--
-- Name: user_assets user_assets_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.user_assets
    ADD CONSTRAINT user_assets_pkey PRIMARY KEY (asid);


--
-- Name: user_chat_unread_counts user_chat_unread_counts_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.user_chat_unread_counts
    ADD CONSTRAINT user_chat_unread_counts_pkey PRIMARY KEY (ucid);


--
-- Name: user_equipments user_equipments_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.user_equipments
    ADD CONSTRAINT user_equipments_pkey PRIMARY KEY (id);


--
-- Name: user_player_tracks user_player_tracks_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.user_player_tracks
    ADD CONSTRAINT user_player_tracks_pkey PRIMARY KEY (tid);


--
-- Name: user_players user_players_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.user_players
    ADD CONSTRAINT user_players_pkey PRIMARY KEY (id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: tfaadmin
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (uuid);


--
-- Name: type_ctime; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX type_ctime ON public.link_accounts USING btree (account_type, created_time);


--
-- PostgreSQL database dump complete
--

