use ::c2rust_bitfields;
use std::sync::atomic::{AtomicUsize, Ordering};
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type XML_ParserStruct;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn XML_SetElementDeclHandler(parser: XML_Parser, eldecl: XML_ElementDeclHandler);
    fn XML_SetAttlistDeclHandler(parser: XML_Parser, attdecl: XML_AttlistDeclHandler);
    fn XML_SetXmlDeclHandler(parser: XML_Parser, xmldecl: XML_XmlDeclHandler);
    fn XML_ParserCreate(encoding: *const XML_Char) -> XML_Parser;
    fn XML_ParserCreate_MM(
        encoding: *const XML_Char,
        memsuite: *const XML_Memory_Handling_Suite,
        namespaceSeparator: *const XML_Char,
    ) -> XML_Parser;
    fn XML_ParserReset(parser: XML_Parser, encoding: *const XML_Char) -> XML_Bool;
    fn XML_SetEntityDeclHandler(parser: XML_Parser, handler: XML_EntityDeclHandler);
    fn XML_SetElementHandler(
        parser: XML_Parser,
        start: XML_StartElementHandler,
        end: XML_EndElementHandler,
    );
    fn XML_SetStartElementHandler(parser: XML_Parser, handler: XML_StartElementHandler);
    fn XML_SetEndElementHandler(parser: XML_Parser, handler: XML_EndElementHandler);
    fn XML_SetCharacterDataHandler(parser: XML_Parser, handler: XML_CharacterDataHandler);
    fn XML_SetProcessingInstructionHandler(
        parser: XML_Parser,
        handler: XML_ProcessingInstructionHandler,
    );
    fn XML_SetCommentHandler(parser: XML_Parser, handler: XML_CommentHandler);
    fn XML_SetStartCdataSectionHandler(parser: XML_Parser, start: XML_StartCdataSectionHandler);
    fn XML_SetEndCdataSectionHandler(parser: XML_Parser, end: XML_EndCdataSectionHandler);
    fn XML_SetDefaultHandler(parser: XML_Parser, handler: XML_DefaultHandler);
    fn XML_SetDefaultHandlerExpand(parser: XML_Parser, handler: XML_DefaultHandler);
    fn XML_SetDoctypeDeclHandler(
        parser: XML_Parser,
        start: XML_StartDoctypeDeclHandler,
        end: XML_EndDoctypeDeclHandler,
    );
    fn XML_SetStartDoctypeDeclHandler(parser: XML_Parser, start: XML_StartDoctypeDeclHandler);
    fn XML_SetEndDoctypeDeclHandler(parser: XML_Parser, end: XML_EndDoctypeDeclHandler);
    fn XML_SetNotationDeclHandler(parser: XML_Parser, handler: XML_NotationDeclHandler);
    fn XML_SetNotStandaloneHandler(parser: XML_Parser, handler: XML_NotStandaloneHandler);
    fn XML_SetExternalEntityRefHandler(parser: XML_Parser, handler: XML_ExternalEntityRefHandler);
    fn XML_SetExternalEntityRefHandlerArg(parser: XML_Parser, arg: *mut ::core::ffi::c_void);
    fn XML_SetSkippedEntityHandler(parser: XML_Parser, handler: XML_SkippedEntityHandler);
    fn XML_SetUnknownEncodingHandler(
        parser: XML_Parser,
        handler: XML_UnknownEncodingHandler,
        encodingHandlerData: *mut ::core::ffi::c_void,
    );
    fn XML_SetUserData(parser: XML_Parser, userData: *mut ::core::ffi::c_void);
    fn XML_SetEncoding(parser: XML_Parser, encoding: *const XML_Char) -> XML_Status;
    fn XML_UseParserAsHandlerArg(parser: XML_Parser);
    fn XML_UseForeignDTD(parser: XML_Parser, useDTD: XML_Bool) -> XML_Error;
    fn XML_SetBase(parser: XML_Parser, base: *const XML_Char) -> XML_Status;
    fn XML_GetBase(parser: XML_Parser) -> *const XML_Char;
    fn XML_Parse(
        parser: XML_Parser,
        s: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        isFinal: ::core::ffi::c_int,
    ) -> XML_Status;
    fn XML_GetBuffer(parser: XML_Parser, len: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn XML_ParseBuffer(
        parser: XML_Parser,
        len: ::core::ffi::c_int,
        isFinal: ::core::ffi::c_int,
    ) -> XML_Status;
    fn XML_ResumeParser(parser: XML_Parser) -> XML_Status;
    fn XML_GetParsingStatus(parser: XML_Parser, status: *mut XML_ParsingStatus);
    fn XML_ExternalEntityParserCreate(
        parser: XML_Parser,
        context: *const XML_Char,
        encoding: *const XML_Char,
    ) -> XML_Parser;
    fn XML_SetParamEntityParsing(
        parser: XML_Parser,
        parsing: XML_ParamEntityParsing,
    ) -> ::core::ffi::c_int;
    fn XML_SetHashSalt(parser: XML_Parser, hash_salt: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn XML_GetErrorCode(parser: XML_Parser) -> XML_Error;
    fn XML_GetCurrentLineNumber(parser: XML_Parser) -> XML_Size;
    fn XML_GetCurrentColumnNumber(parser: XML_Parser) -> XML_Size;
    fn XML_GetCurrentByteIndex(parser: XML_Parser) -> XML_Index;
    fn XML_GetCurrentByteCount(parser: XML_Parser) -> ::core::ffi::c_int;
    fn XML_GetInputContext(
        parser: XML_Parser,
        offset: *mut ::core::ffi::c_int,
        size: *mut ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn XML_FreeContentModel(parser: XML_Parser, model: *mut XML_Content);
    fn XML_MemMalloc(parser: XML_Parser, size: size_t) -> *mut ::core::ffi::c_void;
    fn XML_MemRealloc(
        parser: XML_Parser,
        ptr: *mut ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn XML_MemFree(parser: XML_Parser, ptr: *mut ::core::ffi::c_void);
    fn XML_ParserFree(parser: XML_Parser);
    fn XML_ErrorString(code: XML_Error) -> *const XML_LChar;
    fn XML_GetFeatureList() -> *const XML_Feature;
    fn XML_SetAllocTrackerActivationThreshold(
        parser: XML_Parser,
        activationThresholdBytes: ::core::ffi::c_ulonglong,
    ) -> XML_Bool;
    fn XML_SetReparseDeferralEnabled(parser: XML_Parser, enabled: XML_Bool) -> XML_Bool;
    fn _INTERNAL_trim_to_complete_utf8_characters(
        from: *const ::core::ffi::c_char,
        fromLimRef: *mut *const ::core::ffi::c_char,
    );
    static mut g_reparseDeferralEnabledDefault: XML_Bool;
    static mut g_bytesScanned: ::core::ffi::c_uint;
    fn set_subtest(fmt: *const ::core::ffi::c_char, ...);
    fn _check_set_test_info(
        function: *const ::core::ffi::c_char,
        filename: *const ::core::ffi::c_char,
        lineno: ::core::ffi::c_int,
    );
    fn _fail(
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
    ) -> !;
    fn tcase_create(name: *const ::core::ffi::c_char) -> *mut TCase;
    fn suite_add_tcase(suite: *mut Suite, tc: *mut TCase);
    fn tcase_add_checked_fixture(
        tc: *mut TCase,
        setup: tcase_setup_function,
        teardown: tcase_teardown_function,
    );
    fn tcase_add_test(tc: *mut TCase, test: tcase_test_function);
    fn StructData_Init(storage: *mut StructData);
    fn StructData_CheckItems(
        storage: *mut StructData,
        expected: *const StructDataEntry,
        count: ::core::ffi::c_int,
    );
    fn StructData_Dispose(storage: *mut StructData);
    fn CharData_Init(storage: *mut CharData);
    fn CharData_CheckXMLChars(storage: *mut CharData, s: *const XML_Char) -> ::core::ffi::c_int;
    static mut g_parser: XML_Parser;
    static mut g_resumable: XML_Bool;
    static mut g_abortable: XML_Bool;
    static mut g_chunkSize: ::core::ffi::c_int;
    static mut long_character_data_text: *const ::core::ffi::c_char;
    static mut long_cdata_text: *const ::core::ffi::c_char;
    static mut get_buffer_test_text: *const ::core::ffi::c_char;
    fn tcase_add_test__ifdef_xml_dtd(tc: *mut TCase, test: tcase_test_function);
    fn tcase_add_test__if_xml_ge(tc: *mut TCase, test: tcase_test_function);
    fn basic_teardown();
    fn _xml_failure(parser: XML_Parser, file: *const ::core::ffi::c_char, line: ::core::ffi::c_int);
    fn _XML_Parse_SINGLE_BYTES(
        parser: XML_Parser,
        s: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        isFinal: ::core::ffi::c_int,
    ) -> XML_Status;
    fn _expect_failure(
        text: *const ::core::ffi::c_char,
        errorCode: XML_Error,
        errorMessage: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        lineno: ::core::ffi::c_int,
    );
    fn _run_character_check(
        text: *const ::core::ffi::c_char,
        expected: *const XML_Char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
    );
    fn _run_attribute_check(
        text: *const ::core::ffi::c_char,
        expected: *const XML_Char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
    );
    fn _run_ext_character_check(
        text: *const ::core::ffi::c_char,
        test_data: *mut ExtTest,
        expected: *const XML_Char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
    );
    fn init_dummy_handlers();
    fn get_dummy_handler_flags() -> ::core::ffi::c_ulong;
    fn dummy_xdecl_handler(
        userData: *mut ::core::ffi::c_void,
        version: *const XML_Char,
        encoding: *const XML_Char,
        standalone: ::core::ffi::c_int,
    );
    fn dummy_start_doctype_handler(
        userData: *mut ::core::ffi::c_void,
        doctypeName: *const XML_Char,
        sysid: *const XML_Char,
        pubid: *const XML_Char,
        has_internal_subset: ::core::ffi::c_int,
    );
    fn dummy_end_doctype_handler(userData: *mut ::core::ffi::c_void);
    fn dummy_entity_decl_handler(
        userData: *mut ::core::ffi::c_void,
        entityName: *const XML_Char,
        is_parameter_entity: ::core::ffi::c_int,
        value: *const XML_Char,
        value_length: ::core::ffi::c_int,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
        notationName: *const XML_Char,
    );
    fn dummy_notation_decl_handler(
        userData: *mut ::core::ffi::c_void,
        notationName: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    );
    fn dummy_element_decl_handler(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        model: *mut XML_Content,
    );
    fn dummy_attlist_decl_handler(
        userData: *mut ::core::ffi::c_void,
        elname: *const XML_Char,
        attname: *const XML_Char,
        att_type: *const XML_Char,
        dflt: *const XML_Char,
        isrequired: ::core::ffi::c_int,
    );
    fn dummy_comment_handler(userData: *mut ::core::ffi::c_void, data: *const XML_Char);
    fn dummy_pi_handler(
        userData: *mut ::core::ffi::c_void,
        target: *const XML_Char,
        data: *const XML_Char,
    );
    fn dummy_start_element(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        atts: *mut *const XML_Char,
    );
    fn dummy_end_element(userData: *mut ::core::ffi::c_void, name: *const XML_Char);
    fn dummy_start_cdata_handler(userData: *mut ::core::ffi::c_void);
    fn dummy_end_cdata_handler(userData: *mut ::core::ffi::c_void);
    fn dummy_cdata_handler(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn dummy_default_handler(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn dummy_skip_handler(
        userData: *mut ::core::ffi::c_void,
        entityName: *const XML_Char,
        is_parameter_entity: ::core::ffi::c_int,
    );
    static mut g_handler_data: *const ::core::ffi::c_void;
    static mut g_comment_count: ::core::ffi::c_int;
    static mut g_skip_count: ::core::ffi::c_int;
    static mut g_xdecl_count: ::core::ffi::c_int;
    fn start_element_event_handler(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        atts: *mut *const XML_Char,
    );
    fn end_element_event_handler(userData: *mut ::core::ffi::c_void, name: *const XML_Char);
    fn start_element_event_handler2(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        attr: *mut *const XML_Char,
    );
    fn end_element_event_handler2(userData: *mut ::core::ffi::c_void, name: *const XML_Char);
    fn counting_start_element_handler(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        atts: *mut *const XML_Char,
    );
    fn suspending_end_handler(userData: *mut ::core::ffi::c_void, s: *const XML_Char);
    fn start_element_suspender(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        atts: *mut *const XML_Char,
    );
    fn UnknownEncodingHandler(
        data: *mut ::core::ffi::c_void,
        encoding: *const XML_Char,
        info: *mut XML_Encoding,
    ) -> ::core::ffi::c_int;
    fn UnrecognisedEncodingHandler(
        data: *mut ::core::ffi::c_void,
        encoding: *const XML_Char,
        info: *mut XML_Encoding,
    ) -> ::core::ffi::c_int;
    fn MiscEncodingHandler(
        data: *mut ::core::ffi::c_void,
        encoding: *const XML_Char,
        info: *mut XML_Encoding,
    ) -> ::core::ffi::c_int;
    fn user_data_checking_unknown_encoding_handler(
        userData: *mut ::core::ffi::c_void,
        encoding: *const XML_Char,
        info: *mut XML_Encoding,
    ) -> ::core::ffi::c_int;
    fn external_entity_loader(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_faulter(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_null_loader(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_resetter(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_suspender(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_suspend_xmldecl(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_suspending_faulter(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_cr_catcher(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_bad_cr_catcher(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_rsqb_catcher(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_good_cdata_ascii(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn entity_suspending_xdecl_handler(
        userData: *mut ::core::ffi::c_void,
        version: *const XML_Char,
        encoding: *const XML_Char,
        standalone: ::core::ffi::c_int,
    );
    fn external_entity_param_checker(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_ref_param_checker(
        parameter: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_param(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_load_ignore(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_load_ignore_utf16(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_load_ignore_utf16_be(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_valuer(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_not_standalone(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_value_aborter(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_public(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_devaluer(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_oneshot_loader(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_loader2(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_faulter2(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_unfinished_attlist(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn reject_not_standalone_handler(userData: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn accept_not_standalone_handler(userData: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn verify_attlist_decl_handler(
        userData: *mut ::core::ffi::c_void,
        element_name: *const XML_Char,
        attr_name: *const XML_Char,
        attr_type: *const XML_Char,
        default_value: *const XML_Char,
        is_required: ::core::ffi::c_int,
    );
    fn clearing_aborting_character_handler(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn parser_stop_character_handler(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn cr_cdata_handler(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn rsqb_handler(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn byte_character_handler(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn ext2_accumulate_characters(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn record_default_handler(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn record_cdata_handler(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn record_cdata_nodefault_handler(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn record_skip_handler(
        userData: *mut ::core::ffi::c_void,
        entityName: *const XML_Char,
        is_parameter_entity: ::core::ffi::c_int,
    );
    fn record_element_start_handler(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        atts: *mut *const XML_Char,
    );
    fn record_element_end_handler(userData: *mut ::core::ffi::c_void, name: *const XML_Char);
    fn _handler_record_get(
        storage: *const handler_record_list,
        index: ::core::ffi::c_int,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
    ) -> *const handler_record_entry;
    fn param_entity_match_handler(
        userData: *mut ::core::ffi::c_void,
        entityName: *const XML_Char,
        is_parameter_entity: ::core::ffi::c_int,
        value: *const XML_Char,
        value_length: ::core::ffi::c_int,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
        notationName: *const XML_Char,
    );
    fn param_entity_match_init(name: *const XML_Char, value: *const XML_Char);
    fn get_param_entity_match_flag() -> ::core::ffi::c_int;
    fn xml_decl_handler(
        userData: *mut ::core::ffi::c_void,
        version: *const XML_Char,
        encoding: *const XML_Char,
        standalone: ::core::ffi::c_int,
    );
    fn param_check_skip_handler(
        userData: *mut ::core::ffi::c_void,
        entityName: *const XML_Char,
        is_parameter_entity: ::core::ffi::c_int,
    );
    fn data_check_comment_handler(userData: *mut ::core::ffi::c_void, data: *const XML_Char);
    fn selective_aborting_default_handler(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn suspending_comment_handler(userData: *mut ::core::ffi::c_void, data: *const XML_Char);
    fn element_decl_suspender(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        model: *mut XML_Content,
    );
    fn accumulate_pi_characters(
        userData: *mut ::core::ffi::c_void,
        target: *const XML_Char,
        data: *const XML_Char,
    );
    fn accumulate_comment(userData: *mut ::core::ffi::c_void, data: *const XML_Char);
    fn accumulate_entity_decl(
        userData: *mut ::core::ffi::c_void,
        entityName: *const XML_Char,
        is_parameter_entity: ::core::ffi::c_int,
        value: *const XML_Char,
        value_length: ::core::ffi::c_int,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
        notationName: *const XML_Char,
    );
    fn accumulate_char_data_and_suspend(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn accumulate_characters(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn accumulate_attribute(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        atts: *mut *const XML_Char,
    );
    fn checking_default_handler(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn accumulate_and_suspend_comment_handler(
        userData: *mut ::core::ffi::c_void,
        data: *const XML_Char,
    );
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type XML_Char = ::core::ffi::c_char;
pub type XML_LChar = ::core::ffi::c_char;
pub type XML_Index = ::core::ffi::c_long;
pub type XML_Size = ::core::ffi::c_ulong;
pub type XML_Parser = *mut XML_ParserStruct;
pub type XML_Bool = ::core::ffi::c_uchar;
pub type XML_Status = ::core::ffi::c_uint;
pub const XML_STATUS_SUSPENDED: XML_Status = 2;
pub const XML_STATUS_OK: XML_Status = 1;
pub const XML_STATUS_ERROR: XML_Status = 0;
pub type XML_Error = ::core::ffi::c_uint;
pub const XML_ERROR_NOT_STARTED: XML_Error = 44;
pub const XML_ERROR_AMPLIFICATION_LIMIT_BREACH: XML_Error = 43;
pub const XML_ERROR_NO_BUFFER: XML_Error = 42;
pub const XML_ERROR_INVALID_ARGUMENT: XML_Error = 41;
pub const XML_ERROR_RESERVED_NAMESPACE_URI: XML_Error = 40;
pub const XML_ERROR_RESERVED_PREFIX_XMLNS: XML_Error = 39;
pub const XML_ERROR_RESERVED_PREFIX_XML: XML_Error = 38;
pub const XML_ERROR_SUSPEND_PE: XML_Error = 37;
pub const XML_ERROR_FINISHED: XML_Error = 36;
pub const XML_ERROR_ABORTED: XML_Error = 35;
pub const XML_ERROR_NOT_SUSPENDED: XML_Error = 34;
pub const XML_ERROR_SUSPENDED: XML_Error = 33;
pub const XML_ERROR_PUBLICID: XML_Error = 32;
pub const XML_ERROR_TEXT_DECL: XML_Error = 31;
pub const XML_ERROR_XML_DECL: XML_Error = 30;
pub const XML_ERROR_INCOMPLETE_PE: XML_Error = 29;
pub const XML_ERROR_UNDECLARING_PREFIX: XML_Error = 28;
pub const XML_ERROR_UNBOUND_PREFIX: XML_Error = 27;
pub const XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING: XML_Error = 26;
pub const XML_ERROR_FEATURE_REQUIRES_XML_DTD: XML_Error = 25;
pub const XML_ERROR_ENTITY_DECLARED_IN_PE: XML_Error = 24;
pub const XML_ERROR_UNEXPECTED_STATE: XML_Error = 23;
pub const XML_ERROR_NOT_STANDALONE: XML_Error = 22;
pub const XML_ERROR_EXTERNAL_ENTITY_HANDLING: XML_Error = 21;
pub const XML_ERROR_UNCLOSED_CDATA_SECTION: XML_Error = 20;
pub const XML_ERROR_INCORRECT_ENCODING: XML_Error = 19;
pub const XML_ERROR_UNKNOWN_ENCODING: XML_Error = 18;
pub const XML_ERROR_MISPLACED_XML_PI: XML_Error = 17;
pub const XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF: XML_Error = 16;
pub const XML_ERROR_BINARY_ENTITY_REF: XML_Error = 15;
pub const XML_ERROR_BAD_CHAR_REF: XML_Error = 14;
pub const XML_ERROR_ASYNC_ENTITY: XML_Error = 13;
pub const XML_ERROR_RECURSIVE_ENTITY_REF: XML_Error = 12;
pub const XML_ERROR_UNDEFINED_ENTITY: XML_Error = 11;
pub const XML_ERROR_PARAM_ENTITY_REF: XML_Error = 10;
pub const XML_ERROR_JUNK_AFTER_DOC_ELEMENT: XML_Error = 9;
pub const XML_ERROR_DUPLICATE_ATTRIBUTE: XML_Error = 8;
pub const XML_ERROR_TAG_MISMATCH: XML_Error = 7;
pub const XML_ERROR_PARTIAL_CHAR: XML_Error = 6;
pub const XML_ERROR_UNCLOSED_TOKEN: XML_Error = 5;
pub const XML_ERROR_INVALID_TOKEN: XML_Error = 4;
pub const XML_ERROR_NO_ELEMENTS: XML_Error = 3;
pub const XML_ERROR_SYNTAX: XML_Error = 2;
pub const XML_ERROR_NO_MEMORY: XML_Error = 1;
pub const XML_ERROR_NONE: XML_Error = 0;
pub type XML_Content_Type = ::core::ffi::c_uint;
pub const XML_CTYPE_SEQ: XML_Content_Type = 6;
pub const XML_CTYPE_CHOICE: XML_Content_Type = 5;
pub const XML_CTYPE_NAME: XML_Content_Type = 4;
pub const XML_CTYPE_MIXED: XML_Content_Type = 3;
pub const XML_CTYPE_ANY: XML_Content_Type = 2;
pub const XML_CTYPE_EMPTY: XML_Content_Type = 1;
pub type XML_Content_Quant = ::core::ffi::c_uint;
pub const XML_CQUANT_PLUS: XML_Content_Quant = 3;
pub const XML_CQUANT_REP: XML_Content_Quant = 2;
pub const XML_CQUANT_OPT: XML_Content_Quant = 1;
pub const XML_CQUANT_NONE: XML_Content_Quant = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_cp {
    pub type_0: XML_Content_Type,
    pub quant: XML_Content_Quant,
    pub name: *mut XML_Char,
    pub numchildren: ::core::ffi::c_uint,
    pub children: *mut XML_Content,
}
pub type XML_Content = XML_cp;
pub type XML_ElementDeclHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, *mut XML_Content) -> ()>;
pub type XML_AttlistDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type XML_XmlDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *const XML_Char,
        ::core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_Memory_Handling_Suite {
    pub malloc_fcn: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>,
    pub realloc_fcn:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>,
    pub free_fcn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
pub type XML_StartElementHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, *mut *const XML_Char) -> (),
>;
pub type XML_EndElementHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> ()>;
pub type XML_CharacterDataHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, ::core::ffi::c_int) -> (),
>;
pub type XML_ProcessingInstructionHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, *const XML_Char) -> ()>;
pub type XML_CommentHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> ()>;
pub type XML_StartCdataSectionHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type XML_EndCdataSectionHandler = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type XML_DefaultHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, ::core::ffi::c_int) -> (),
>;
pub type XML_StartDoctypeDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type XML_EndDoctypeDeclHandler = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type XML_EntityDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        ::core::ffi::c_int,
        *const XML_Char,
        ::core::ffi::c_int,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> (),
>;
pub type XML_NotationDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> (),
>;
pub type XML_NotStandaloneHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type XML_ExternalEntityRefHandler = Option<
    unsafe extern "C" fn(
        XML_Parser,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> ::core::ffi::c_int,
>;
pub type XML_SkippedEntityHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, ::core::ffi::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_Encoding {
    pub map: [::core::ffi::c_int; 256],
    pub data: *mut ::core::ffi::c_void,
    pub convert: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub release: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
pub type XML_UnknownEncodingHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *mut XML_Encoding,
    ) -> ::core::ffi::c_int,
>;
pub type XML_Parsing = ::core::ffi::c_uint;
pub const XML_SUSPENDED: XML_Parsing = 3;
pub const XML_FINISHED: XML_Parsing = 2;
pub const XML_PARSING: XML_Parsing = 1;
pub const XML_INITIALIZED: XML_Parsing = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_ParsingStatus {
    pub parsing: XML_Parsing,
    pub finalBuffer: XML_Bool,
}
pub type XML_ParamEntityParsing = ::core::ffi::c_uint;
pub const XML_PARAM_ENTITY_PARSING_ALWAYS: XML_ParamEntityParsing = 2;
pub const XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE: XML_ParamEntityParsing = 1;
pub const XML_PARAM_ENTITY_PARSING_NEVER: XML_ParamEntityParsing = 0;
pub type XML_FeatureEnum = ::core::ffi::c_uint;
pub const XML_FEATURE_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT: XML_FeatureEnum = 15;
pub const XML_FEATURE_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT: XML_FeatureEnum = 14;
pub const XML_FEATURE_GE: XML_FeatureEnum = 13;
pub const XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT:
    XML_FeatureEnum = 12;
pub const XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT:
    XML_FeatureEnum = 11;
pub const XML_FEATURE_ATTR_INFO: XML_FeatureEnum = 10;
pub const XML_FEATURE_LARGE_SIZE: XML_FeatureEnum = 9;
pub const XML_FEATURE_NS: XML_FeatureEnum = 8;
pub const XML_FEATURE_SIZEOF_XML_LCHAR: XML_FeatureEnum = 7;
pub const XML_FEATURE_SIZEOF_XML_CHAR: XML_FeatureEnum = 6;
pub const XML_FEATURE_MIN_SIZE: XML_FeatureEnum = 5;
pub const XML_FEATURE_CONTEXT_BYTES: XML_FeatureEnum = 4;
pub const XML_FEATURE_DTD: XML_FeatureEnum = 3;
pub const XML_FEATURE_UNICODE_WCHAR_T: XML_FeatureEnum = 2;
pub const XML_FEATURE_UNICODE: XML_FeatureEnum = 1;
pub const XML_FEATURE_END: XML_FeatureEnum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_Feature {
    pub feature: XML_FeatureEnum,
    pub name: *const XML_LChar,
    pub value: ::core::ffi::c_long,
}
pub type ptrdiff_t = isize;
pub type tcase_setup_function = Option<unsafe extern "C" fn() -> ()>;
pub type tcase_teardown_function = Option<unsafe extern "C" fn() -> ()>;
pub type tcase_test_function = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Suite {
    pub name: *const ::core::ffi::c_char,
    pub tests: *mut TCase,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TCase {
    pub name: *const ::core::ffi::c_char,
    pub setup: tcase_setup_function,
    pub teardown: tcase_teardown_function,
    pub tests: *mut tcase_test_function,
    pub ntests: ::core::ffi::c_int,
    pub allocated: ::core::ffi::c_int,
    pub next_tcase: *mut TCase,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StructDataEntry {
    pub str: *const XML_Char,
    pub data0: ::core::ffi::c_int,
    pub data1: ::core::ffi::c_int,
    pub data2: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StructData {
    pub count: ::core::ffi::c_int,
    pub max_count: ::core::ffi::c_int,
    pub entries: *mut StructDataEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharData {
    pub count: ::core::ffi::c_int,
    pub data: [XML_Char; 2048],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtTest {
    pub parse_text: *const ::core::ffi::c_char,
    pub encoding: *const XML_Char,
    pub storage: *mut CharData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attrInfo {
    pub name: *const XML_Char,
    pub value: *const XML_Char,
}
pub type AttrInfo = attrInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elementInfo {
    pub name: *const XML_Char,
    pub attr_count: ::core::ffi::c_int,
    pub id_name: *const XML_Char,
    pub attributes: *mut AttrInfo,
}
pub type ElementInfo = elementInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StructParserAndElementInfo {
    pub parser: XML_Parser,
    pub info: *mut ElementInfo,
}
pub type ParserAndElementInfo = StructParserAndElementInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ext_faults {
    pub parse_text: *const ::core::ffi::c_char,
    pub fail_text: *const ::core::ffi::c_char,
    pub encoding: *const XML_Char,
    pub error: XML_Error,
}
pub type ExtFaults = ext_faults;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ext_hdlr_data {
    pub parse_text: *const ::core::ffi::c_char,
    pub handler: XML_ExternalEntityRefHandler,
    pub storage: *mut CharData,
}
pub type ExtHdlrData = ext_hdlr_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtTest2 {
    pub parse_text: *const ::core::ffi::c_char,
    pub parse_len: ::core::ffi::c_int,
    pub encoding: *const XML_Char,
    pub storage: *mut CharData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtFaults2 {
    pub parse_text: *const ::core::ffi::c_char,
    pub parse_len: ::core::ffi::c_int,
    pub fail_text: *const ::core::ffi::c_char,
    pub encoding: *const XML_Char,
    pub error: XML_Error,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttTest {
    pub definition: *const ::core::ffi::c_char,
    pub element_name: *const XML_Char,
    pub attr_name: *const XML_Char,
    pub attr_type: *const XML_Char,
    pub default_value: *const XML_Char,
    pub is_required: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ByteTestData {
    pub start_element_len: ::core::ffi::c_int,
    pub cdata_len: ::core::ffi::c_int,
    pub total_string_len: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct handler_record_entry {
    pub name: *const ::core::ffi::c_char,
    pub arg: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct handler_record_list {
    pub count: ::core::ffi::c_int,
    pub entries: [handler_record_entry; 50],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct default_check {
    pub expected: *const XML_Char,
    pub expectedLen: ::core::ffi::c_int,
    pub seen: XML_Bool,
}
pub type DefaultCheck = default_check;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParserPlusStorage {
    pub parser: XML_Parser,
    pub storage: *mut CharData,
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = isize;
pub type uintptr_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siphash {
    pub v0: uint64_t,
    pub v1: uint64_t,
    pub v2: uint64_t,
    pub v3: uint64_t,
    pub buf: [::core::ffi::c_uchar; 8],
    pub buf_len: usize,
    pub c: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sipkey {
    pub k: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct element_decl_data {
    pub parser: XML_Parser,
    pub count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub pre: *const ::core::ffi::c_char,
    pub post: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct test_case {
    pub goodName: bool,
    pub goodNameStart: bool,
    pub tagName: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestCase {
    pub doc: *const ::core::ffi::c_char,
    pub expectedStatus: XML_Status,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bom_testdata {
    pub external: *const ::core::ffi::c_char,
    pub split: ::core::ffi::c_int,
    pub nested_callback_happened: XML_Bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaseData {
    pub text_bytes: size_t,
    pub text: *const ::core::ffi::c_char,
    pub expected_error: XML_Error,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaseData_0 {
    pub text: *const ::core::ffi::c_char,
    pub expectedError: XML_Error,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestCase_0 {
    pub doc: *const ::core::ffi::c_char,
    pub usesParameterEntities: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestCase_1 {
    pub expectedMovementInChars: ptrdiff_t,
    pub input: *const ::core::ffi::c_char,
}
pub const XML_CONTEXT_BYTES: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const XML_TRUE: XML_Bool = 1 as ::core::ffi::c_int as XML_Bool;
pub const XML_FALSE: XML_Bool = 0 as ::core::ffi::c_int as XML_Bool;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const DUMMY_NOTATION_DECL_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 3 as ::core::ffi::c_int;
pub const DUMMY_ELEMENT_DECL_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 4 as ::core::ffi::c_int;
pub const DUMMY_SKIP_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 16 as ::core::ffi::c_int;
pub const STRUCT_START_TAG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STRUCT_END_TAG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENTITY_MATCH_FAIL: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const ENTITY_MATCH_NOT_FOUND: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
const BASIC_TESTS_FILE: &[u8] = b"/root/work/expat/tests/basic_tests.c\0";

fn bytes_as_c_char_ptr(bytes: &[u8]) -> *const ::core::ffi::c_char {
    bytes.as_ptr().cast()
}

fn bytes_as_xml_char_ptr(bytes: &[u8]) -> *const XML_Char {
    bytes.as_ptr().cast()
}

fn c_str_from_ptr<'a>(text: *const ::core::ffi::c_char) -> &'a std::ffi::CStr {
    unsafe { std::ffi::CStr::from_ptr(text) }
}

fn slice_from_raw_parts<'a, T>(ptr: *const T, len: usize) -> &'a [T] {
    unsafe { ::core::slice::from_raw_parts(ptr, len) }
}

fn mut_from_ptr<'a, T>(ptr: *mut T) -> &'a mut T {
    unsafe { &mut *ptr }
}

fn value_from_ptr<T: Copy>(ptr: *const T) -> T {
    unsafe { *ptr }
}

fn parser_user_data_ptr(parser: XML_Parser) -> *mut ::core::ffi::c_void {
    unsafe { *(parser as *mut *mut ::core::ffi::c_void) }
}

fn parser_user_data_as<T>(parser: XML_Parser) -> *mut T {
    parser_user_data_ptr(parser).cast()
}

fn parser_user_data_bits(parser: XML_Parser) -> uint32_t {
    parser_user_data_ptr(parser) as uintptr_t as uint32_t
}

struct AttrPairs {
    atts: *mut *const XML_Char,
    index: usize,
}

impl Iterator for AttrPairs {
    type Item = (*const XML_Char, *const XML_Char);

    fn next(&mut self) -> Option<Self::Item> {
        let name = unsafe { *self.atts.add(self.index) };
        if name.is_null() {
            return None;
        }

        let value = unsafe { *self.atts.add(self.index + 1) };
        self.index += 2;
        Some((name, value))
    }
}

fn attr_pairs(atts: *mut *const XML_Char) -> AttrPairs {
    AttrPairs { atts, index: 0 }
}

struct FeatureList {
    current: *const XML_Feature,
}

impl Iterator for FeatureList {
    type Item = &'static XML_Feature;

    fn next(&mut self) -> Option<Self::Item> {
        let feature = unsafe { self.current.as_ref()? };
        if feature.feature as ::core::ffi::c_uint
            == XML_FEATURE_END as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return None;
        }

        self.current = unsafe { self.current.add(1) };
        Some(feature)
    }
}

fn feature_list() -> FeatureList {
    FeatureList {
        current: ffi_call0(XML_GetFeatureList),
    }
}

fn c_string_equals(text: *const ::core::ffi::c_char, expected: &[u8]) -> bool {
    ffi_call2(strcmp, text, bytes_as_c_char_ptr(expected)) == 0 as ::core::ffi::c_int
}

fn xml_string_equals(text: *const XML_Char, expected: &[u8]) -> bool {
    c_string_equals(text.cast(), expected)
}

fn c_string_offset(
    text: *const ::core::ffi::c_char,
    offset: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe { text.offset(offset as isize) }
}

fn set_subtest_message(message: &str) {
    let message = std::ffi::CString::new(message).expect("subtest message must not contain NUL");
    unsafe {
        set_subtest(bytes_as_c_char_ptr(b"%s\0"), message.as_ptr());
    }
}

fn format_attr_normalization_failure(
    buffer: &mut [::core::ffi::c_char; 256],
    attrname: *const XML_Char,
    value: *const XML_Char,
) {
    unsafe {
        snprintf(
            buffer.as_mut_ptr(),
            ::core::mem::size_of_val(buffer) as size_t,
            bytes_as_c_char_ptr(b"attribute value not normalized: %s='%s'\0"),
            attrname,
            value,
        );
    }
}

macro_rules! ffi_call {
    ($function:expr $(, $arg:expr)* $(,)?) => {{
        unsafe { $function($($arg),*) }
    }};
}

macro_rules! unsafe_global_get {
    ($name:ident) => {{
        unsafe { $name }
    }};
}

macro_rules! unsafe_global_set {
    ($name:ident, $value:expr) => {{
        unsafe {
            $name = $value;
        }
    }};
}

fn current_parser() -> XML_Parser {
    unsafe_global_get!(g_parser)
}

fn set_current_parser(parser: XML_Parser) {
    unsafe_global_set!(g_parser, parser);
}

fn current_chunk_size() -> ::core::ffi::c_int {
    unsafe_global_get!(g_chunkSize)
}

#[derive(Copy, Clone)]
enum SharedTestText {
    CharacterData,
    Cdata,
}

fn shared_test_text(kind: SharedTestText) -> *const ::core::ffi::c_char {
    match kind {
        SharedTestText::CharacterData => unsafe_global_get!(long_character_data_text),
        SharedTestText::Cdata => unsafe_global_get!(long_cdata_text),
    }
}

fn set_parser_stop_state(resumable: XML_Bool, abortable: Option<XML_Bool>) {
    unsafe_global_set!(g_resumable, resumable);
    if let Some(abortable) = abortable {
        unsafe_global_set!(g_abortable, abortable);
    }
}

fn ffi_call1<A, R>(function: unsafe extern "C" fn(A) -> R, a: A) -> R {
    ffi_call!(function, a)
}

fn ffi_call0<R>(function: unsafe extern "C" fn() -> R) -> R {
    ffi_call!(function)
}

fn ffi_call2<A, B, R>(function: unsafe extern "C" fn(A, B) -> R, a: A, b: B) -> R {
    ffi_call!(function, a, b)
}

fn ffi_call3<A, B, C, R>(function: unsafe extern "C" fn(A, B, C) -> R, a: A, b: B, c: C) -> R {
    ffi_call!(function, a, b, c)
}

fn ffi_call4<A, B, C, D, R>(
    function: unsafe extern "C" fn(A, B, C, D) -> R,
    a: A,
    b: B,
    c: C,
    d: D,
) -> R {
    ffi_call!(function, a, b, c, d)
}

fn ffi_call5<A, B, C, D, E, R>(
    function: unsafe extern "C" fn(A, B, C, D, E) -> R,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
) -> R {
    ffi_call!(function, a, b, c, d, e)
}

fn set_test_info(name: &[u8], line: ::core::ffi::c_int) {
    ffi_call3(
        _check_set_test_info,
        bytes_as_c_char_ptr(name),
        bytes_as_c_char_ptr(BASIC_TESTS_FILE),
        line,
    );
}

fn fail_test(line: ::core::ffi::c_int, message: &[u8]) -> ! {
    ffi_call3(
        _fail,
        bytes_as_c_char_ptr(BASIC_TESTS_FILE),
        line,
        bytes_as_c_char_ptr(message),
    )
}

fn fail_test_with_buffer(line: ::core::ffi::c_int, message: *mut ::core::ffi::c_char) -> ! {
    ffi_call3(_fail, bytes_as_c_char_ptr(BASIC_TESTS_FILE), line, message)
}

fn fail_test_message(line: ::core::ffi::c_int, message: String) -> ! {
    let message = std::ffi::CString::new(message).expect("failure message must not contain NUL");
    ffi_call3(
        _fail,
        bytes_as_c_char_ptr(BASIC_TESTS_FILE),
        line,
        message.as_ptr(),
    )
}

fn xml_failure(line: ::core::ffi::c_int) {
    ffi_call3(
        _xml_failure,
        current_parser(),
        bytes_as_c_char_ptr(BASIC_TESTS_FILE),
        line,
    );
}

fn parser_create() -> XML_Parser {
    ffi_call1(XML_ParserCreate, ::core::ptr::null::<XML_Char>())
}

fn parse_single_bytes(text: *const ::core::ffi::c_char, len: ::core::ffi::c_int) -> XML_Status {
    ffi_call4(
        _XML_Parse_SINGLE_BYTES,
        current_parser(),
        text,
        len,
        XML_TRUE as ::core::ffi::c_int,
    )
}

fn parse_single_bytes_c_string(text: *const ::core::ffi::c_char) -> XML_Status {
    parse_single_bytes(text, c_string_len(text))
}

fn parser_error_code() -> XML_Error {
    ffi_call1(XML_GetErrorCode, current_parser())
}

fn parser_status_is_error(status: XML_Status) -> bool {
    status as ::core::ffi::c_uint == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
}

fn ensure_parser_success(status: XML_Status, line: ::core::ffi::c_int) {
    if parser_status_is_error(status) {
        xml_failure(line);
    }
}

fn ensure_parser_success_for(parser: XML_Parser, status: XML_Status, line: ::core::ffi::c_int) {
    if parser_status_is_error(status) {
        xml_failure_for(parser, line);
    }
}

fn ensure_parser_error(status: XML_Status, line: ::core::ffi::c_int) {
    if !parser_status_is_error(status) {
        fail_test(line, b"Expected a parse error\0");
    }
}

fn parser_current_line_number() -> XML_Size {
    ffi_call1(XML_GetCurrentLineNumber, current_parser())
}

fn parser_current_column_number() -> XML_Size {
    ffi_call1(XML_GetCurrentColumnNumber, current_parser())
}

fn parser_reset() {
    ffi_call2(
        XML_ParserReset,
        current_parser(),
        ::core::ptr::null::<XML_Char>(),
    );
}

fn parser_set_hash_salt(hash_salt: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    ffi_call2(XML_SetHashSalt, current_parser(), hash_salt)
}

fn parser_parse(
    text: *const ::core::ffi::c_char,
    len: ::core::ffi::c_int,
    is_final: ::core::ffi::c_int,
) -> XML_Status {
    ffi_call4(XML_Parse, current_parser(), text, len, is_final)
}

fn parser_parse_c_string(text: *const ::core::ffi::c_char) -> XML_Status {
    parser_parse(text, c_string_len(text), XML_TRUE as ::core::ffi::c_int)
}

fn parser_set_character_data_handler(handler: XML_CharacterDataHandler) {
    ffi_call2(XML_SetCharacterDataHandler, current_parser(), handler);
}

fn parser_set_start_element_handler(handler: XML_StartElementHandler) {
    ffi_call2(XML_SetStartElementHandler, current_parser(), handler);
}

fn parser_set_end_element_handler(handler: XML_EndElementHandler) {
    ffi_call2(XML_SetEndElementHandler, current_parser(), handler);
}

fn parser_set_default_handler(handler: XML_DefaultHandler) {
    ffi_call2(XML_SetDefaultHandler, current_parser(), handler);
}

fn parser_set_start_cdata_section_handler(handler: XML_StartCdataSectionHandler) {
    ffi_call2(XML_SetStartCdataSectionHandler, current_parser(), handler);
}

fn parser_set_end_cdata_section_handler(handler: XML_EndCdataSectionHandler) {
    ffi_call2(XML_SetEndCdataSectionHandler, current_parser(), handler);
}

fn parser_set_xml_decl_handler(handler: XML_XmlDeclHandler) {
    ffi_call2(XML_SetXmlDeclHandler, current_parser(), handler);
}

fn parser_set_external_entity_ref_handler(handler: XML_ExternalEntityRefHandler) {
    ffi_call2(XML_SetExternalEntityRefHandler, current_parser(), handler);
}

fn parser_set_not_standalone_handler(handler: XML_NotStandaloneHandler) {
    ffi_call2(XML_SetNotStandaloneHandler, current_parser(), handler);
}

fn parser_set_unknown_encoding_handler(
    handler: XML_UnknownEncodingHandler,
    encoding_handler_data: *mut ::core::ffi::c_void,
) {
    ffi_call3(
        XML_SetUnknownEncodingHandler,
        current_parser(),
        handler,
        encoding_handler_data,
    );
}

fn parser_set_unknown_encoding_handler_for(
    parser: XML_Parser,
    handler: XML_UnknownEncodingHandler,
    encoding_handler_data: *mut ::core::ffi::c_void,
) {
    ffi_call3(
        XML_SetUnknownEncodingHandler,
        parser,
        handler,
        encoding_handler_data,
    );
}

fn parser_set_user_data(user_data: *mut ::core::ffi::c_void) {
    ffi_call2(XML_SetUserData, current_parser(), user_data);
}

fn parser_set_user_data_for(parser: XML_Parser, user_data: *mut ::core::ffi::c_void) {
    ffi_call2(XML_SetUserData, parser, user_data);
}

fn parser_set_encoding(encoding: *const XML_Char) -> XML_Status {
    ffi_call2(XML_SetEncoding, current_parser(), encoding)
}

fn parser_set_param_entity_parsing(parsing: XML_ParamEntityParsing) -> ::core::ffi::c_int {
    ffi_call2(XML_SetParamEntityParsing, current_parser(), parsing)
}

fn parser_set_param_entity_parsing_for(
    parser: XML_Parser,
    parsing: XML_ParamEntityParsing,
) -> ::core::ffi::c_int {
    ffi_call2(XML_SetParamEntityParsing, parser, parsing)
}

fn parser_use_foreign_dtd(use_dtd: XML_Bool) -> XML_Error {
    ffi_call2(XML_UseForeignDTD, current_parser(), use_dtd)
}

fn parser_set_base(base: *const XML_Char) -> XML_Status {
    ffi_call2(XML_SetBase, current_parser(), base)
}

fn parser_base() -> *const XML_Char {
    ffi_call1(XML_GetBase, current_parser())
}

fn parse_single_bytes_with_final(
    text: *const ::core::ffi::c_char,
    len: ::core::ffi::c_int,
    is_final: ::core::ffi::c_int,
) -> XML_Status {
    ffi_call4(
        _XML_Parse_SINGLE_BYTES,
        current_parser(),
        text,
        len,
        is_final,
    )
}

fn parse_single_bytes_with_final_for(
    parser: XML_Parser,
    text: *const ::core::ffi::c_char,
    len: ::core::ffi::c_int,
    is_final: ::core::ffi::c_int,
) -> XML_Status {
    ffi_call4(_XML_Parse_SINGLE_BYTES, parser, text, len, is_final)
}

fn parse_single_bytes_c_string_for(
    parser: XML_Parser,
    text: *const ::core::ffi::c_char,
) -> XML_Status {
    parse_single_bytes_with_final_for(
        parser,
        text,
        c_string_len(text),
        XML_TRUE as ::core::ffi::c_int,
    )
}

fn parser_reset_for(parser: XML_Parser) {
    ffi_call2(XML_ParserReset, parser, ::core::ptr::null::<XML_Char>());
}

fn parser_set_external_entity_ref_handler_for(
    parser: XML_Parser,
    handler: XML_ExternalEntityRefHandler,
) {
    ffi_call2(XML_SetExternalEntityRefHandler, parser, handler);
}

fn parser_resume() -> XML_Status {
    ffi_call1(XML_ResumeParser, current_parser())
}

fn parser_parsing_status() -> XML_ParsingStatus {
    let mut status = XML_ParsingStatus {
        parsing: XML_INITIALIZED,
        finalBuffer: 0,
    };
    ffi_call2(
        XML_GetParsingStatus,
        current_parser(),
        &mut status as *mut XML_ParsingStatus,
    );
    status
}

fn configure_resumable_character_data_handler(resumable: XML_Bool) {
    set_parser_stop_state(resumable, None);
    parser_set_character_data_handler(clearing_aborting_character_data_handler());
}

fn configure_subordinate_external_entity_parser(
    handler: XML_ExternalEntityRefHandler,
    resumable: Option<XML_Bool>,
) {
    parser_set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
    parser_set_external_entity_ref_handler(handler);
    if let Some(resumable) = resumable {
        set_parser_stop_state(resumable, None);
    }
}

fn assert_status_ok(status: XML_Status, line: ::core::ffi::c_int, message: &[u8]) {
    if status as ::core::ffi::c_uint != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        fail_test(line, message);
    }
}

fn parse_single_bytes_buffer(text: &[u8], is_final: ::core::ffi::c_int) -> XML_Status {
    let len = ::core::ffi::c_int::try_from(text.len().saturating_sub(1))
        .expect("buffer length should fit into c_int");
    parse_single_bytes_with_final(text.as_ptr().cast(), len, is_final)
}

fn char_data_init(storage: &mut CharData) {
    ffi_call1(CharData_Init, storage as *mut CharData);
}

fn char_data_check_xml_chars(storage: &mut CharData, expected: *const XML_Char) {
    ffi_call2(CharData_CheckXMLChars, storage as *mut CharData, expected);
}

fn set_up_accumulating_character_storage(storage: &mut CharData) {
    char_data_init(storage);
    parser_set_user_data((storage as *mut CharData).cast());
    parser_set_character_data_handler(accumulating_character_handler());
}

fn expect_character_data_from_single_bytes(
    text: &[u8],
    expected: *const XML_Char,
    line: ::core::ffi::c_int,
) {
    let mut storage = CharData {
        count: 0,
        data: [0; 2048],
    };

    set_up_accumulating_character_storage(&mut storage);
    ensure_parser_success(
        parse_single_bytes_buffer(text, XML_TRUE as ::core::ffi::c_int),
        line,
    );
    char_data_check_xml_chars(&mut storage, expected);
}

fn expect_character_data_from_parse_buffer(
    text: &[u8],
    expected: *const XML_Char,
    alloc_line: ::core::ffi::c_int,
    parse_line: ::core::ffi::c_int,
) {
    let mut storage = CharData {
        count: 0,
        data: [0; 2048],
    };
    let len = ::core::ffi::c_int::try_from(text.len().saturating_sub(1))
        .expect("buffer length should fit into c_int");

    set_up_accumulating_character_storage(&mut storage);

    let buffer = parser_get_buffer(len);
    if buffer.is_null() {
        fail_test(alloc_line, b"Could not allocate parse buffer\0");
    }

    copy_buffer_from_c_string(buffer, text.as_ptr().cast(), len);
    ensure_parser_success(
        parser_parse_buffer(len, XML_TRUE as ::core::ffi::c_int),
        parse_line,
    );
    char_data_check_xml_chars(&mut storage, expected);
}

fn struct_data_init(storage: &mut StructData) {
    ffi_call1(StructData_Init, storage as *mut StructData);
}

fn struct_data_check_items(storage: &mut StructData, expected: &[StructDataEntry]) {
    let count = ::core::ffi::c_int::try_from(expected.len())
        .expect("expected entry count should fit into c_int");
    ffi_call3(
        StructData_CheckItems,
        storage as *mut StructData,
        expected.as_ptr(),
        count,
    );
}

fn struct_data_dispose(storage: &mut StructData) {
    ffi_call1(StructData_Dispose, storage as *mut StructData);
}

fn run_attribute_check(
    text: *const ::core::ffi::c_char,
    expected: *const XML_Char,
    line: ::core::ffi::c_int,
) {
    ffi_call4(
        _run_attribute_check,
        text,
        expected,
        bytes_as_c_char_ptr(BASIC_TESTS_FILE),
        line,
    );
}

fn run_ext_character_check(
    text: *const ::core::ffi::c_char,
    test_data: &mut ExtTest,
    expected: *const XML_Char,
    line: ::core::ffi::c_int,
) {
    ffi_call5(
        _run_ext_character_check,
        text,
        test_data as *mut ExtTest,
        expected,
        bytes_as_c_char_ptr(BASIC_TESTS_FILE),
        line,
    );
}

fn accumulating_character_handler() -> XML_CharacterDataHandler {
    Some(
        accumulate_characters
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const XML_Char,
                ::core::ffi::c_int,
            ) -> (),
    )
}

fn default_handler() -> XML_DefaultHandler {
    Some(
        dummy_default_handler
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const XML_Char,
                ::core::ffi::c_int,
            ) -> (),
    )
}

fn accumulating_default_handler() -> XML_DefaultHandler {
    Some(
        accumulate_characters
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const XML_Char,
                ::core::ffi::c_int,
            ) -> (),
    )
}

fn xml_decl_handler_for_tests() -> XML_XmlDeclHandler {
    Some(
        dummy_xdecl_handler
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const XML_Char,
                *const XML_Char,
                ::core::ffi::c_int,
            ) -> (),
    )
}

fn unknown_encoding_handler_for_tests() -> XML_UnknownEncodingHandler {
    Some(UnknownEncodingHandler)
}

fn misc_encoding_handler_for_tests() -> XML_UnknownEncodingHandler {
    Some(MiscEncodingHandler)
}

fn unrecognised_encoding_handler_for_tests() -> XML_UnknownEncodingHandler {
    Some(UnrecognisedEncodingHandler)
}

fn user_data_checking_unknown_encoding_handler_for_tests() -> XML_UnknownEncodingHandler {
    Some(user_data_checking_unknown_encoding_handler)
}

fn external_entity_loader_handler_for_tests() -> XML_ExternalEntityRefHandler {
    Some(external_entity_loader)
}

fn external_entity_loader2_handler_for_tests() -> XML_ExternalEntityRefHandler {
    Some(external_entity_loader2)
}

fn external_entity_faulter_handler_for_tests() -> XML_ExternalEntityRefHandler {
    Some(external_entity_faulter)
}

fn external_entity_resetter_handler_for_tests() -> XML_ExternalEntityRefHandler {
    Some(external_entity_resetter)
}

fn external_entity_suspender_handler_for_tests() -> XML_ExternalEntityRefHandler {
    Some(external_entity_suspender)
}

fn external_entity_suspend_xmldecl_handler_for_tests() -> XML_ExternalEntityRefHandler {
    Some(external_entity_suspend_xmldecl)
}

fn external_entity_suspending_faulter_handler_for_tests() -> XML_ExternalEntityRefHandler {
    Some(external_entity_suspending_faulter)
}

fn external_entity_good_cdata_handler_for_tests() -> XML_ExternalEntityRefHandler {
    Some(external_entity_good_cdata_ascii)
}

fn external_entity_null_loader_handler_for_tests() -> XML_ExternalEntityRefHandler {
    Some(external_entity_null_loader)
}

fn reject_not_standalone_handler_for_tests() -> XML_NotStandaloneHandler {
    Some(reject_not_standalone_handler)
}

fn accept_not_standalone_handler_for_tests() -> XML_NotStandaloneHandler {
    Some(accept_not_standalone_handler)
}

fn clearing_aborting_character_data_handler() -> XML_CharacterDataHandler {
    Some(
        clearing_aborting_character_handler
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const XML_Char,
                ::core::ffi::c_int,
            ) -> (),
    )
}

fn parser_stop_character_data_handler() -> XML_CharacterDataHandler {
    Some(
        parser_stop_character_handler
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const XML_Char,
                ::core::ffi::c_int,
            ) -> (),
    )
}

fn start_element_event_handler2_for_tests() -> XML_StartElementHandler {
    Some(
        start_element_event_handler2
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const XML_Char,
                *mut *const XML_Char,
            ) -> (),
    )
}

fn record_element_start_handler_for_tests() -> XML_StartElementHandler {
    Some(
        record_element_start_handler
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const XML_Char,
                *mut *const XML_Char,
            ) -> (),
    )
}

fn end_element_event_handler2_for_tests() -> XML_EndElementHandler {
    Some(
        end_element_event_handler2
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
    )
}

fn end_element_event_handler_for_tests() -> XML_EndElementHandler {
    Some(
        end_element_event_handler
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
    )
}

fn dummy_cdata_handler_for_tests() -> XML_CharacterDataHandler {
    Some(
        dummy_cdata_handler
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const XML_Char,
                ::core::ffi::c_int,
            ) -> (),
    )
}

fn start_cdata_handler_for_tests() -> XML_StartCdataSectionHandler {
    Some(dummy_start_cdata_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
}

fn end_cdata_handler_for_tests() -> XML_EndCdataSectionHandler {
    Some(dummy_end_cdata_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
}

fn attr_whitespace_handler_for_tests() -> XML_StartElementHandler {
    Some(
        check_attr_contains_normalized_whitespace
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const XML_Char,
                *mut *const XML_Char,
            ) -> (),
    )
}

fn c_string_len(text: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    ffi_call1(strlen, text) as ::core::ffi::c_int
}

fn buffer_test_text() -> *const ::core::ffi::c_char {
    unsafe_global_get!(get_buffer_test_text)
}

fn parser_free(parser: XML_Parser) {
    ffi_call1(XML_ParserFree, parser);
}

fn parser_error_code_for(parser: XML_Parser) -> XML_Error {
    ffi_call1(XML_GetErrorCode, parser)
}

fn external_entity_parser_create(parent: XML_Parser) -> XML_Parser {
    ffi_call3(
        XML_ExternalEntityParserCreate,
        parent,
        ::core::ptr::null::<XML_Char>(),
        ::core::ptr::null::<XML_Char>(),
    )
}

fn parser_parse_for(
    parser: XML_Parser,
    text: *const ::core::ffi::c_char,
    len: ::core::ffi::c_int,
    is_final: ::core::ffi::c_int,
) -> XML_Status {
    ffi_call4(XML_Parse, parser, text, len, is_final)
}

fn parser_buffer_for(parser: XML_Parser, len: ::core::ffi::c_int) -> *mut ::core::ffi::c_void {
    ffi_call2(XML_GetBuffer, parser, len)
}

fn current_parser_buffer(len: ::core::ffi::c_int) -> *mut ::core::ffi::c_void {
    parser_buffer_for(current_parser(), len)
}

fn parser_parse_buffer_for(
    parser: XML_Parser,
    len: ::core::ffi::c_int,
    is_final: ::core::ffi::c_int,
) -> XML_Status {
    ffi_call3(XML_ParseBuffer, parser, len, is_final)
}

fn current_parser_parse_buffer(
    len: ::core::ffi::c_int,
    is_final: ::core::ffi::c_int,
) -> XML_Status {
    parser_parse_buffer_for(current_parser(), len, is_final)
}

fn parser_current_byte_index() -> XML_Index {
    ffi_call1(XML_GetCurrentByteIndex, current_parser())
}

fn parser_current_byte_count() -> ::core::ffi::c_int {
    ffi_call1(XML_GetCurrentByteCount, current_parser())
}

fn parser_input_context(
    offset: &mut ::core::ffi::c_int,
    size: &mut ::core::ffi::c_int,
) -> *const XML_Char {
    ffi_call3(
        XML_GetInputContext,
        current_parser(),
        offset as *mut ::core::ffi::c_int,
        size as *mut ::core::ffi::c_int,
    )
}

fn parser_set_alloc_tracker_activation_threshold(
    parser: XML_Parser,
    activation_threshold_bytes: ::core::ffi::c_ulonglong,
) -> XML_Bool {
    ffi_call2(
        XML_SetAllocTrackerActivationThreshold,
        parser,
        activation_threshold_bytes,
    )
}

fn copy_c_string_to_buffer(buffer: *mut ::core::ffi::c_void, text: *const ::core::ffi::c_char) {
    ffi_call3(memcpy, buffer, text.cast(), c_string_len(text) as size_t);
}

fn create_parser_or_fail(line: ::core::ffi::c_int) -> XML_Parser {
    let parser = parser_create();
    if parser.is_null() {
        fail_test(line, b"check failed: parser != NULL\0");
    }
    parser
}

fn create_external_entity_parser_or_fail(
    parent: XML_Parser,
    line: ::core::ffi::c_int,
) -> XML_Parser {
    let parser = external_entity_parser_create(parent);
    if parser.is_null() {
        fail_test(line, b"check failed: ext_parser != NULL\0");
    }
    parser
}

fn xml_failure_for(parser: XML_Parser, line: ::core::ffi::c_int) {
    ffi_call3(
        _xml_failure,
        parser,
        bytes_as_c_char_ptr(BASIC_TESTS_FILE),
        line,
    );
}

fn byte_character_handler_for_tests() -> XML_CharacterDataHandler {
    Some(
        byte_character_handler
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const XML_Char,
                ::core::ffi::c_int,
            ) -> (),
    )
}

fn set_subtest_quoted_text(text: *const ::core::ffi::c_char) {
    set_subtest_message(&format!("\"{}\"", c_string_lossy(text)));
}

fn set_subtest_with_len_first(first_len: ::core::ffi::c_int) {
    set_subtest_message(&format!("with len={} first", first_len));
}

fn set_subtest_text(text: *const ::core::ffi::c_char) {
    set_subtest_message(&c_string_lossy(text));
}

fn set_subtest_case_number(case_number: usize) {
    set_subtest_message(&format!("case {}", case_number));
}

fn xml_error_string(error: XML_Error) -> *const XML_LChar {
    ffi_call1(XML_ErrorString, error)
}

fn c_string_lossy(text: *const ::core::ffi::c_char) -> String {
    c_str_from_ptr(text).to_string_lossy().into_owned()
}

fn xml_error_string_lossy(error: XML_Error) -> String {
    c_string_lossy(xml_error_string(error).cast())
}

macro_rules! xml_name_eq {
    ($actual:expr, $expected:expr) => {{
        ffi_call2(
            strcmp,
            ($actual).cast::<::core::ffi::c_char>(),
            bytes_as_c_char_ptr($expected),
        ) == 0 as ::core::ffi::c_int
    }};
}

fn parser_get_buffer(len: ::core::ffi::c_int) -> *mut ::core::ffi::c_void {
    ffi_call2(XML_GetBuffer, current_parser(), len)
}

fn parser_parse_buffer(len: ::core::ffi::c_int, is_final: ::core::ffi::c_int) -> XML_Status {
    ffi_call3(XML_ParseBuffer, current_parser(), len, is_final)
}

fn copy_buffer_from_c_string(
    dest: *mut ::core::ffi::c_void,
    src: *const ::core::ffi::c_char,
    len: ::core::ffi::c_int,
) {
    ffi_call3(
        memcpy,
        dest,
        src.cast::<::core::ffi::c_void>(),
        len as size_t,
    );
}

fn cstr(bytes: &'static [u8]) -> &'static std::ffi::CStr {
    std::ffi::CStr::from_bytes_with_nul(bytes).expect("test literal must be NUL terminated")
}

fn assert_test_condition(condition: bool, line: ::core::ffi::c_int, message: &[u8]) {
    if !condition {
        fail_test(line, message);
    }
}

fn assert_xml_size_eq(actual: XML_Size, expected: XML_Size, unit: &str, line: ::core::ffi::c_int) {
    if actual != expected {
        fail_test_message(line, format!("expected {expected} {unit}, saw {actual}"));
    }
}

fn expect_failure(
    text: *const ::core::ffi::c_char,
    error_code: XML_Error,
    error_message: &[u8],
    line: ::core::ffi::c_int,
) {
    ffi_call5(
        _expect_failure,
        text,
        error_code,
        bytes_as_c_char_ptr(error_message),
        bytes_as_c_char_ptr(BASIC_TESTS_FILE),
        line,
    );
}

fn run_character_check(
    text: *const ::core::ffi::c_char,
    expected: *const XML_Char,
    line: ::core::ffi::c_int,
) {
    ffi_call4(
        _run_character_check,
        text,
        expected,
        bytes_as_c_char_ptr(BASIC_TESTS_FILE),
        line,
    );
}

fn write_illegal_utf8_input(buffer: &mut [u8; 100], ordinal: ::core::ffi::c_int) {
    ffi_call!(
        snprintf,
        buffer.as_mut_ptr().cast(),
        buffer.len() as size_t,
        bytes_as_c_char_ptr(b"<e>%ccd</e>\0"),
        ordinal,
    );
}

fn write_illegal_utf8_failure_message(buffer: &mut [u8; 100], ordinal: ::core::ffi::c_int) {
    ffi_call!(
        snprintf,
        buffer.as_mut_ptr().cast(),
        buffer.len() as size_t,
        bytes_as_c_char_ptr(b"expected token error for '%c' (ordinal %d) in UTF-8 text\0"),
        ordinal,
        ordinal,
    );
}

fn load_sip_u64(bytes: &[::core::ffi::c_uchar]) -> uint64_t {
    let bytes: [::core::ffi::c_uchar; 8] = bytes
        .try_into()
        .expect("SipHash chunks should always be 8 bytes");
    uint64_t::from_le_bytes(bytes)
}

fn sip_tokey<'a>(key: &'a mut sipkey, src: &[::core::ffi::c_uchar; 16]) -> &'a mut sipkey {
    key.k[0] = load_sip_u64(&src[..8]);
    key.k[1] = load_sip_u64(&src[8..]);
    key
}

fn sip_round(state: &mut siphash, rounds: ::core::ffi::c_int) {
    for _ in 0..usize::try_from(rounds).expect("SipHash round count should be non-negative") {
        state.v0 = state.v0.wrapping_add(state.v1);
        state.v1 = state.v1.rotate_left(13);
        state.v1 ^= state.v0;
        state.v0 = state.v0.rotate_left(32);
        state.v2 = state.v2.wrapping_add(state.v3);
        state.v3 = state.v3.rotate_left(16);
        state.v3 ^= state.v2;
        state.v0 = state.v0.wrapping_add(state.v3);
        state.v3 = state.v3.rotate_left(21);
        state.v3 ^= state.v0;
        state.v2 = state.v2.wrapping_add(state.v1);
        state.v1 = state.v1.rotate_left(17);
        state.v1 ^= state.v2;
        state.v2 = state.v2.rotate_left(32);
    }
}

fn sip24_init<'a>(state: &'a mut siphash, key: &sipkey) -> &'a mut siphash {
    state.v0 = ((0x736f6d65 as ::core::ffi::c_uint as uint64_t) << 32) | 0x70736575 as uint64_t;
    state.v0 ^= key.k[0];
    state.v1 = ((0x646f7261 as ::core::ffi::c_uint as uint64_t) << 32) | 0x6e646f6d as uint64_t;
    state.v1 ^= key.k[1];
    state.v2 = ((0x6c796765 as ::core::ffi::c_uint as uint64_t) << 32) | 0x6e657261 as uint64_t;
    state.v2 ^= key.k[0];
    state.v3 = ((0x74656462 as ::core::ffi::c_uint as uint64_t) << 32) | 0x79746573 as uint64_t;
    state.v3 ^= key.k[1];
    state.buf_len = 0;
    state.c = 0;
    state
}

fn sip24_update<'a>(state: &'a mut siphash, mut src: &[::core::ffi::c_uchar]) -> &'a mut siphash {
    if state.buf_len != 0 {
        let to_copy = (state.buf.len() - state.buf_len).min(src.len());
        state.buf[state.buf_len..state.buf_len + to_copy].copy_from_slice(&src[..to_copy]);
        state.buf_len += to_copy;
        src = &src[to_copy..];
        if state.buf_len == state.buf.len() {
            let m = load_sip_u64(&state.buf);
            state.v3 ^= m;
            sip_round(state, 2);
            state.v0 ^= m;
            state.buf_len = 0;
            state.c = state.c.wrapping_add(state.buf.len() as uint64_t);
        }
    }

    while src.len() >= state.buf.len() {
        let chunk_len = state.buf.len();
        let m = load_sip_u64(&src[..chunk_len]);
        state.v3 ^= m;
        sip_round(state, 2);
        state.v0 ^= m;
        state.c = state.c.wrapping_add(chunk_len as uint64_t);
        src = &src[chunk_len..];
    }

    if !src.is_empty() {
        state.buf[..src.len()].copy_from_slice(src);
        state.buf_len = src.len();
    }

    state
}

fn sip24_final(state: &mut siphash) -> uint64_t {
    let mut b = state.c.wrapping_add(state.buf_len as uint64_t) << 56;
    for (index, byte) in state.buf[..state.buf_len].iter().enumerate() {
        b |= (*byte as uint64_t) << (index * 8);
    }

    state.v3 ^= b;
    sip_round(state, 2);
    state.v0 ^= b;
    state.v2 ^= 0xff;
    sip_round(state, 4);
    state.v0 ^ state.v1 ^ state.v2 ^ state.v3
}

fn siphash24(src: &[::core::ffi::c_uchar], key: &sipkey) -> uint64_t {
    let mut state = siphash {
        v0: 0,
        v1: 0,
        v2: 0,
        v3: 0,
        buf: [0; 8],
        buf_len: 0,
        c: 0,
    };
    sip24_final(sip24_update(sip24_init(&mut state, key), src))
}

fn sip24_valid() -> ::core::ffi::c_int {
    static VECTORS: [[::core::ffi::c_uchar; 8]; 64] = [
        [
            0x31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xdd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x47 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xdb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x72 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xfd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x67 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xdc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x39 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x74 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x5a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x4f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x2d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x7e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xfb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x96 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x67 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x85 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xb7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x71 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcf as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x8d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x64 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xce as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x58 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcb as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x37 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xab as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x62 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xb0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x82 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9e as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xdd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x7a as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xa7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xfb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x86 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x75 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x84 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x56 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xea as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x7a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xe5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x49 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x61 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xdb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x47 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x69 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x9c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x96 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x4b as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xbd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x61 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x98 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbe as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xc7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x67 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x88 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x95 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x67 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xc8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xce as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xaf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x49 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xea as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x85 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xde as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbc as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x63 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x61 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2f as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xa5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xac as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x4d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xde as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x71 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x65 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x95 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xef as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x49 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x42 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x41 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xfa as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xce as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x7c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x72 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x51 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x71 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xe3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x78 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x59 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x38 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xae as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x97 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x34 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xb4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x4d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x81 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x39 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x62 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x4d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xdc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9a as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xd0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x59 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x58 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x42 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x95 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x75 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xdb as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xeb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x75 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x98 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x51 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x96 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xaf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x72 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x52 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x97 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x43 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x64 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x5a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xb2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x74 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x6f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x81 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xea as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xec as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xbd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x83 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xaf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x65 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xea as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xe1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x91 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xec as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6c as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x96 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x62 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9f as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x38 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x72 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xeb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x4c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x95 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
    ];
    let mut input = [0; 64];
    let mut key = sipkey { k: [0; 2] };
    sip_tokey(
        &mut key,
        b"\0\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0B\x0C\r\x0E\x0F",
    );
    for i in 0..input.len() {
        input[i] =
            ::core::ffi::c_uchar::try_from(i).expect("SipHash test input index should fit into u8");
        if siphash24(&input[..i], &key) != load_sip_u64(&VECTORS[i]) {
            return 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
extern "C" fn basic_setup() {
    set_current_parser(parser_create());
    if current_parser().is_null() {
        fail_test(75 as ::core::ffi::c_int, b"Parser not created.\0");
    }
}
extern "C" fn test_nul_byte() {
    set_test_info(b"test_nul_byte\0", 82 as ::core::ffi::c_int);
    let text = *b"<doc>\0</doc>\0";
    if parse_single_bytes(text.as_ptr().cast(), (text.len() - 1) as ::core::ffi::c_int)
        as ::core::ffi::c_uint
        == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            88 as ::core::ffi::c_int,
            b"Parser did not report error on NUL-byte.\0",
        );
    }
    if parser_error_code() as ::core::ffi::c_uint
        != XML_ERROR_INVALID_TOKEN as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(90 as ::core::ffi::c_int);
    }
}
extern "C" fn test_u0000_char() {
    set_test_info(b"test_u0000_char\0", 94 as ::core::ffi::c_int);
    expect_failure(
        bytes_as_c_char_ptr(b"<doc>&#0;</doc>\0"),
        XML_ERROR_BAD_CHAR_REF,
        b"Parser did not report error on NUL-byte.\0",
        97 as ::core::ffi::c_int,
    );
}
extern "C" fn test_siphash_self() {
    set_test_info(b"test_siphash_self\0", 101 as ::core::ffi::c_int);
    if sip24_valid() == 0 {
        fail_test(103 as ::core::ffi::c_int, b"SipHash self-test failed\0");
    }
}
extern "C" fn test_siphash_spec() {
    set_test_info(b"test_siphash_spec\0", 107 as ::core::ffi::c_int);
    let message: [::core::ffi::c_uchar; 16] =
        *b"\0\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0B\x0C\r\x0E\0";
    let len = message.len() - 1;
    let expected: uint64_t = (0xa129ca61 as ::core::ffi::c_uint as uint64_t)
        << 32 as ::core::ffi::c_int
        | 0x49be45e5 as uint64_t;
    let mut state: siphash = siphash {
        v0: 0,
        v1: 0,
        v2: 0,
        v3: 0,
        buf: [0; 8],
        buf_len: 0,
        c: 0,
    };
    let mut key: sipkey = sipkey { k: [0; 2] };
    sip_tokey(
        &mut key,
        b"\0\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0B\x0C\r\x0E\x0F",
    );
    sip24_init(&mut state, &key);
    sip24_update(&mut state, &message[..4]);
    sip24_update(&mut state, &message[4..len]);
    sip24_update(&mut state, &message[..0]);
    if sip24_final(&mut state) != expected {
        fail_test(
            128 as ::core::ffi::c_int,
            b"sip24_final failed spec test\n\0",
        );
    }
    if siphash24(&message[..len], &key) != expected {
        fail_test(132 as ::core::ffi::c_int, b"siphash24 failed spec test\n\0");
    }
}
extern "C" fn test_bom_utf8() {
    set_test_info(b"test_bom_utf8\0", 136 as ::core::ffi::c_int);
    if parse_single_bytes_c_string(bytes_as_c_char_ptr(b"\xEF\xBB\xBF<e/>\0"))
        as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(142 as ::core::ffi::c_int);
    }
}
extern "C" fn test_bom_utf16_be() {
    set_test_info(b"test_bom_utf16_be\0", 146 as ::core::ffi::c_int);
    let text = *b"\xFE\xFF\0<\0e\0/\0>\0";
    if parse_single_bytes(text.as_ptr().cast(), (text.len() - 1) as ::core::ffi::c_int)
        as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(151 as ::core::ffi::c_int);
    }
}
extern "C" fn test_bom_utf16_le() {
    set_test_info(b"test_bom_utf16_le\0", 155 as ::core::ffi::c_int);
    let text = *b"\xFF\xFE<\0e\0/\0>\0\0";
    if parse_single_bytes(text.as_ptr().cast(), (text.len() - 1) as ::core::ffi::c_int)
        as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(160 as ::core::ffi::c_int);
    }
}
extern "C" fn test_nobom_utf16_le() {
    set_test_info(b"test_nobom_utf16_le\0", 164 as ::core::ffi::c_int);
    let text = *b" \0<\0e\0/\0>\0\0";
    if current_chunk_size() == 1 as ::core::ffi::c_int {
        return;
    }
    if parse_single_bytes(text.as_ptr().cast(), (text.len() - 1) as ::core::ffi::c_int)
        as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(175 as ::core::ffi::c_int);
    }
}
extern "C" fn test_hash_collision() {
    set_test_info(b"test_hash_collision\0", 179 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(
        b"<doc>\n<a1/><a2/><a3/><a4/><a5/><a6/><a7/><a8/>\n<b1></b1><b2 attr='foo'>This is a foo</b2><b3></b3><b4></b4>\n<b5></b5><b6></b6><b7></b7><b8></b8>\n<c1/><c2/><c3/><c4/><c5/><c6/><c7/><c8/>\n<d1/><d2/><d3/><d4/><d5/><d6/><d7/>\n<d8>This triggers the table growth and collides with b2</d8>\n</doc>\n\0",
    );
    parser_set_hash_salt(
        ((0xffffffff as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xff99fc90 as uint64_t) as ::core::ffi::c_ulong,
    );
    if parse_single_bytes_c_string(text) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(202 as ::core::ffi::c_int);
    }
}
extern "C" fn test_danish_latin1() {
    set_test_info(b"test_danish_latin1\0", 208 as ::core::ffi::c_int);
    run_character_check(
        bytes_as_c_char_ptr(b"<?xml version='1.0' encoding='iso-8859-1'?>\n<e>J\xF8rgen \xE6\xF8\xE5\xC6\xD8\xC5</e>\0"),
        b"J\xC3\xB8rgen \xC3\xA6\xC3\xB8\xC3\xA5\xC3\x86\xC3\x98\xC3\x85\0".as_ptr()
            as *const XML_Char,
        218 as ::core::ffi::c_int,
    );
}
extern "C" fn test_french_charref_hexidecimal() {
    set_test_info(
        b"test_french_charref_hexidecimal\0",
        223 as ::core::ffi::c_int,
    );
    run_character_check(
        bytes_as_c_char_ptr(
            b"<?xml version='1.0' encoding='iso-8859-1'?>\n<doc>&#xE9;&#xE8;&#xE0;&#xE7;&#xEA;&#xC8;</doc>\0",
        ),
        b"\xC3\xA9\xC3\xA8\xC3\xA0\xC3\xA7\xC3\xAA\xC3\x88\0".as_ptr() as *const XML_Char,
        232 as ::core::ffi::c_int,
    );
}
extern "C" fn test_french_charref_decimal() {
    set_test_info(b"test_french_charref_decimal\0", 236 as ::core::ffi::c_int);
    run_character_check(
        bytes_as_c_char_ptr(
            b"<?xml version='1.0' encoding='iso-8859-1'?>\n<doc>&#233;&#232;&#224;&#231;&#234;&#200;</doc>\0",
        ),
        b"\xC3\xA9\xC3\xA8\xC3\xA0\xC3\xA7\xC3\xAA\xC3\x88\0".as_ptr() as *const XML_Char,
        245 as ::core::ffi::c_int,
    );
}
extern "C" fn test_french_latin1() {
    set_test_info(b"test_french_latin1\0", 249 as ::core::ffi::c_int);
    run_character_check(
        bytes_as_c_char_ptr(
            b"<?xml version='1.0' encoding='iso-8859-1'?>\n<doc>\xE9\xE8\xE0\xE7\xEA\xC8</doc>\0",
        ),
        b"\xC3\xA9\xC3\xA8\xC3\xA0\xC3\xA7\xC3\xAA\xC3\x88\0".as_ptr() as *const XML_Char,
        258 as ::core::ffi::c_int,
    );
}
extern "C" fn test_french_utf8() {
    set_test_info(b"test_french_utf8\0", 262 as ::core::ffi::c_int);
    run_character_check(
        bytes_as_c_char_ptr(b"<?xml version='1.0' encoding='utf-8'?>\n<doc>\xC3\xA9</doc>\0"),
        b"\xC3\xA9\0".as_ptr() as *const XML_Char,
        270 as ::core::ffi::c_int,
    );
}
extern "C" fn test_utf8_false_rejection() {
    set_test_info(b"test_utf8_false_rejection\0", 279 as ::core::ffi::c_int);
    run_character_check(
        bytes_as_c_char_ptr(b"<doc>\xEF\xBA\xBF</doc>\0"),
        b"\xEF\xBA\xBF\0".as_ptr() as *const XML_Char,
        286 as ::core::ffi::c_int,
    );
}
extern "C" fn test_illegal_utf8() {
    set_test_info(b"test_illegal_utf8\0", 295 as ::core::ffi::c_int);
    let mut text = [0_u8; 100];
    let mut i = 128 as ::core::ffi::c_int;
    while i <= 255 as ::core::ffi::c_int {
        write_illegal_utf8_input(&mut text, i);
        if parse_single_bytes(text.as_ptr().cast(), c_string_len(text.as_ptr().cast()))
            as ::core::ffi::c_uint
            == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            write_illegal_utf8_failure_message(&mut text, i);
            fail_test_with_buffer(306 as ::core::ffi::c_int, text.as_mut_ptr().cast());
        } else if parser_error_code() as ::core::ffi::c_uint
            != XML_ERROR_INVALID_TOKEN as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            xml_failure(308 as ::core::ffi::c_int);
        }
        parser_reset();
        i += 1;
    }
}
pub const UTF8_LEAD_1: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"\x7F\0") };
pub const UTF8_LEAD_2: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"\xDF\0") };
pub const UTF8_LEAD_3: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"\xEF\0") };
pub const UTF8_LEAD_4: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"\xF7\0") };
extern "C" fn test_utf8_auto_align() {
    unsafe {
        _check_set_test_info(
            b"test_utf8_auto_align\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            322 as ::core::ffi::c_int,
        );
        let mut cases: [TestCase_1; 11] = [
            TestCase_1 {
                expectedMovementInChars: 0 as ptrdiff_t,
                input: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            TestCase_1 {
                expectedMovementInChars: 0 as ptrdiff_t,
                input: UTF8_LEAD_1.as_ptr(),
            },
            TestCase_1 {
                expectedMovementInChars: -(1 as ::core::ffi::c_int) as ptrdiff_t,
                input: UTF8_LEAD_2.as_ptr(),
            },
            TestCase_1 {
                expectedMovementInChars: 0 as ptrdiff_t,
                input: b"\xDF\xBF\0".as_ptr() as *const ::core::ffi::c_char,
            },
            TestCase_1 {
                expectedMovementInChars: -(1 as ::core::ffi::c_int) as ptrdiff_t,
                input: UTF8_LEAD_3.as_ptr(),
            },
            TestCase_1 {
                expectedMovementInChars: -(2 as ::core::ffi::c_int) as ptrdiff_t,
                input: b"\xEF\xBF\0".as_ptr() as *const ::core::ffi::c_char,
            },
            TestCase_1 {
                expectedMovementInChars: 0 as ptrdiff_t,
                input: b"\xEF\xBF\xBF\0".as_ptr() as *const ::core::ffi::c_char,
            },
            TestCase_1 {
                expectedMovementInChars: -(1 as ::core::ffi::c_int) as ptrdiff_t,
                input: UTF8_LEAD_4.as_ptr(),
            },
            TestCase_1 {
                expectedMovementInChars: -(2 as ::core::ffi::c_int) as ptrdiff_t,
                input: b"\xF7\xBF\0".as_ptr() as *const ::core::ffi::c_char,
            },
            TestCase_1 {
                expectedMovementInChars: -(3 as ::core::ffi::c_int) as ptrdiff_t,
                input: b"\xF7\xBF\xBF\0".as_ptr() as *const ::core::ffi::c_char,
            },
            TestCase_1 {
                expectedMovementInChars: 0 as ptrdiff_t,
                input: b"\xF7\xBF\xBF\xBF\0".as_ptr() as *const ::core::ffi::c_char,
            },
        ];
        let mut i: size_t = 0 as size_t;
        let mut success: bool = true_0 != 0;
        while i
            < (::core::mem::size_of::<[TestCase_1; 11]>() as usize)
                .wrapping_div(::core::mem::size_of::<TestCase_1>() as usize)
        {
            let mut fromLim: *const ::core::ffi::c_char = cases[i as usize]
                .input
                .offset(strlen(cases[i as usize].input) as isize);
            let fromLimInitially: *const ::core::ffi::c_char = fromLim;
            let mut actualMovementInChars: ptrdiff_t = 0;
            _INTERNAL_trim_to_complete_utf8_characters(cases[i as usize].input, &raw mut fromLim);
            actualMovementInChars =
                fromLim.offset_from(fromLimInitially) as ::core::ffi::c_long as ptrdiff_t;
            if actualMovementInChars != cases[i as usize].expectedMovementInChars {
                let mut j: size_t = 0 as size_t;
                success = false_0 != 0;
                printf(
                    b"[-] UTF-8 case %2u: Expected movement by %2d chars, actually moved by %2d chars: \"\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    i.wrapping_add(1 as size_t) as ::core::ffi::c_uint,
                    cases[i as usize].expectedMovementInChars as ::core::ffi::c_int,
                    actualMovementInChars as ::core::ffi::c_int,
                );
                while j < strlen(cases[i as usize].input) {
                    printf(
                        b"\\x%02x\0".as_ptr() as *const ::core::ffi::c_char,
                        *cases[i as usize].input.offset(j as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int,
                    );
                    j = j.wrapping_add(1);
                }
                printf(b"\"\n\0".as_ptr() as *const ::core::ffi::c_char);
            }
            i = i.wrapping_add(1);
        }
        if !success {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                371 as ::core::ffi::c_int,
                b"UTF-8 auto-alignment is not bullet-proof\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_utf16() {
    set_test_info(b"test_utf16\0", 376 as ::core::ffi::c_int);
    let text = *b"\0<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\x001\0.\x000\0'\0 \0e\0n\0c\0o\0d\0i\0n\0g\0=\0'\0U\0T\0F\0-\x001\x006\0'\0?\0>\0\n\0<\0d\0o\0c\0 \0a\0=\0'\x001\x002\x003\0'\0>\0s\0o\0m\0e\0 \xFF!\0 \0t\0e\0x\0t\0<\0/\0d\0o\0c\0>\0";
    let expected = bytes_as_xml_char_ptr(b"some \xEF\xBC\xA1 text\0");
    let mut storage = CharData {
        count: 0,
        data: [0; 2048],
    };

    char_data_init(&mut storage);
    parser_set_user_data((&mut storage as *mut CharData).cast());
    parser_set_character_data_handler(accumulating_character_handler());

    if parse_single_bytes_buffer(&text, XML_TRUE as ::core::ffi::c_int) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(402 as ::core::ffi::c_int);
    }

    char_data_check_xml_chars(&mut storage, expected);
}
extern "C" fn test_utf16_le_epilog_newline() {
    set_test_info(b"test_utf16_le_epilog_newline\0", 407 as ::core::ffi::c_int);
    let first_chunk_bytes: usize = 17;
    let text = *b"\xFF\xFE<\0e\0/\0>\0\r\0\n\0\r\0\n\0\0";

    if first_chunk_bytes >= text.len().saturating_sub(1) {
        fail_test(
            414 as ::core::ffi::c_int,
            b"bad value of first_chunk_bytes\0",
        );
    }

    if parse_single_bytes_with_final(
        text.as_ptr().cast(),
        first_chunk_bytes as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(417 as ::core::ffi::c_int);
        return;
    }

    let remaining = &text[first_chunk_bytes..text.len() - 1];
    if parse_single_bytes_with_final(
        remaining.as_ptr().cast(),
        remaining.len() as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(424 as ::core::ffi::c_int);
    }
}
extern "C" fn test_not_utf16() {
    set_test_info(b"test_not_utf16\0", 430 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(b"<?xml version='1.0' encoding='utf-16'?><doc>Hi</doc>\0");

    parser_set_xml_decl_handler(xml_decl_handler_for_tests());
    expect_failure(
        text,
        XML_ERROR_INCORRECT_ENCODING,
        b"UTF-16 declared in UTF-8 not faulted\0",
        437 as ::core::ffi::c_int,
    );
}
extern "C" fn test_bad_encoding() {
    set_test_info(b"test_bad_encoding\0", 442 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(b"<doc>Hi</doc>\0");

    if parser_set_encoding(bytes_as_xml_char_ptr(b"unknown-encoding\0")) as u64 == 0 {
        fail_test(446 as ::core::ffi::c_int, b"XML_SetEncoding failed\0");
    }

    expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Unknown encoding not faulted\0",
        448 as ::core::ffi::c_int,
    );
}
extern "C" fn test_latin1_umlauts() {
    set_test_info(b"test_latin1_umlauts\0", 453 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='iso-8859-1'?>\n<e a='\xE4 \xF6 \xFC &#228; &#246; &#252; &#x00E4; &#x0F6; &#xFC; >'\n  >\xE4 \xF6 \xFC &#228; &#246; &#252; &#x00E4; &#x0F6; &#xFC; ></e>\0",
    );
    let expected = bytes_as_xml_char_ptr(
        b"\xC3\xA4 \xC3\xB6 \xC3\xBC \xC3\xA4 \xC3\xB6 \xC3\xBC \xC3\xA4 \xC3\xB6 \xC3\xBC >\0",
    );

    run_character_check(text, expected, 468 as ::core::ffi::c_int);
    parser_reset();
    run_attribute_check(text, expected, 470 as ::core::ffi::c_int);
    parser_reset();
    parser_set_default_handler(default_handler());
    run_character_check(text, expected, 474 as ::core::ffi::c_int);
    parser_reset();
    parser_set_default_handler(default_handler());
    run_attribute_check(text, expected, 477 as ::core::ffi::c_int);
}
extern "C" fn test_long_utf8_character() {
    set_test_info(b"test_long_utf8_character\0", 482 as ::core::ffi::c_int);
    let text =
        bytes_as_c_char_ptr(b"<?xml version='1.0' encoding='utf-8'?>\n<do\xF0\x90\x80\x80/>\0");

    expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"4-byte UTF-8 character in element name not faulted\0",
        488 as ::core::ffi::c_int,
    );
}
extern "C" fn test_long_latin1_attribute() {
    set_test_info(b"test_long_latin1_attribute\0", 495 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='iso-8859-1'?>\n<doc att='ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO\xE4'>\n</doc>\0",
    );
    let expected = bytes_as_xml_char_ptr(
        b"ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO\xC3\xA4\0",
    );

    run_attribute_check(text, expected, 545 as ::core::ffi::c_int);
}
extern "C" fn test_long_ascii_attribute() {
    set_test_info(b"test_long_ascii_attribute\0", 552 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='us-ascii'?>\n<doc att='ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP01234'>\n</doc>\0",
    );
    let expected = bytes_as_xml_char_ptr(
        b"ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP01234\0",
    );

    run_attribute_check(text, expected, 596 as ::core::ffi::c_int);
}
extern "C" fn test_line_number_after_parse() {
    set_test_info(b"test_line_number_after_parse\0", 601 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(b"<tag>\n\n\n</tag>\0");

    ensure_parser_success(parse_single_bytes_c_string(text), 609 as ::core::ffi::c_int);
    assert_xml_size_eq(
        parser_current_line_number(),
        4 as XML_Size,
        "lines",
        615 as ::core::ffi::c_int,
    );
}

extern "C" fn test_column_number_after_parse() {
    set_test_info(
        b"test_column_number_after_parse\0",
        621 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(b"<tag></tag>\0");

    ensure_parser_success(parse_single_bytes_c_string(text), 627 as ::core::ffi::c_int);
    assert_xml_size_eq(
        parser_current_column_number(),
        11 as XML_Size,
        "columns",
        633 as ::core::ffi::c_int,
    );
}

extern "C" fn test_line_and_column_numbers_inside_handlers() {
    set_test_info(
        b"test_line_and_column_numbers_inside_handlers\0",
        639 as ::core::ffi::c_int,
    );
    let text =
        bytes_as_c_char_ptr(b"<a>\n  <b>\r\n    <c/>\r  </b>\n  <d>\n    <f/>\n  </d>\n</a>\0");
    let expected = [
        StructDataEntry {
            str: b"a\0".as_ptr() as *const XML_Char,
            data0: 0 as ::core::ffi::c_int,
            data1: 1 as ::core::ffi::c_int,
            data2: STRUCT_START_TAG,
        },
        StructDataEntry {
            str: b"b\0".as_ptr() as *const XML_Char,
            data0: 2 as ::core::ffi::c_int,
            data1: 2 as ::core::ffi::c_int,
            data2: STRUCT_START_TAG,
        },
        StructDataEntry {
            str: b"c\0".as_ptr() as *const XML_Char,
            data0: 4 as ::core::ffi::c_int,
            data1: 3 as ::core::ffi::c_int,
            data2: STRUCT_START_TAG,
        },
        StructDataEntry {
            str: b"c\0".as_ptr() as *const XML_Char,
            data0: 8 as ::core::ffi::c_int,
            data1: 3 as ::core::ffi::c_int,
            data2: STRUCT_END_TAG,
        },
        StructDataEntry {
            str: b"b\0".as_ptr() as *const XML_Char,
            data0: 2 as ::core::ffi::c_int,
            data1: 4 as ::core::ffi::c_int,
            data2: STRUCT_END_TAG,
        },
        StructDataEntry {
            str: b"d\0".as_ptr() as *const XML_Char,
            data0: 2 as ::core::ffi::c_int,
            data1: 5 as ::core::ffi::c_int,
            data2: STRUCT_START_TAG,
        },
        StructDataEntry {
            str: b"f\0".as_ptr() as *const XML_Char,
            data0: 4 as ::core::ffi::c_int,
            data1: 6 as ::core::ffi::c_int,
            data2: STRUCT_START_TAG,
        },
        StructDataEntry {
            str: b"f\0".as_ptr() as *const XML_Char,
            data0: 8 as ::core::ffi::c_int,
            data1: 6 as ::core::ffi::c_int,
            data2: STRUCT_END_TAG,
        },
        StructDataEntry {
            str: b"d\0".as_ptr() as *const XML_Char,
            data0: 2 as ::core::ffi::c_int,
            data1: 7 as ::core::ffi::c_int,
            data2: STRUCT_END_TAG,
        },
        StructDataEntry {
            str: b"a\0".as_ptr() as *const XML_Char,
            data0: 0 as ::core::ffi::c_int,
            data1: 8 as ::core::ffi::c_int,
            data2: STRUCT_END_TAG,
        },
    ];
    let mut storage = StructData {
        count: 0,
        max_count: 0,
        entries: ::core::ptr::null_mut::<StructDataEntry>(),
    };

    struct_data_init(&mut storage);
    parser_set_user_data((&mut storage as *mut StructData).cast());
    parser_set_start_element_handler(start_element_event_handler2_for_tests());
    parser_set_end_element_handler(end_element_event_handler2_for_tests());
    ensure_parser_success(parse_single_bytes_c_string(text), 663 as ::core::ffi::c_int);
    struct_data_check_items(&mut storage, &expected);
    struct_data_dispose(&mut storage);
}

extern "C" fn test_line_number_after_error() {
    set_test_info(b"test_line_number_after_error\0", 671 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(b"<a>\n  <b>\n  </a>\0");

    ensure_parser_error(parse_single_bytes_c_string(text), 678 as ::core::ffi::c_int);
    assert_xml_size_eq(
        parser_current_line_number(),
        3 as XML_Size,
        "lines",
        685 as ::core::ffi::c_int,
    );
}

extern "C" fn test_column_number_after_error() {
    set_test_info(
        b"test_column_number_after_error\0",
        691 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(b"<a>\n  <b>\n  </a>\0");

    ensure_parser_error(parse_single_bytes_c_string(text), 698 as ::core::ffi::c_int);
    assert_xml_size_eq(
        parser_current_column_number(),
        4 as XML_Size,
        "columns",
        705 as ::core::ffi::c_int,
    );
}

extern "C" fn test_really_long_lines() {
    set_test_info(b"test_really_long_lines\0", 711 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(
        b"<e>ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+</e>\0",
    );

    ensure_parser_success(parse_single_bytes_c_string(text), 741 as ::core::ffi::c_int);
}

extern "C" fn test_really_long_encoded_lines() {
    set_test_info(
        b"test_really_long_encoded_lines\0",
        746 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='iso-8859-1'?><e>ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+</e>\0",
    );
    let parse_len = c_string_len(text);

    parser_set_character_data_handler(dummy_cdata_handler_for_tests());
    let buffer = parser_get_buffer(parse_len);
    if buffer.is_null() {
        fail_test(
            781 as ::core::ffi::c_int,
            b"Could not allocate parse buffer\0",
        );
    }
    copy_buffer_from_c_string(buffer, text, parse_len);
    ensure_parser_success(
        parser_parse_buffer(parse_len, XML_TRUE as ::core::ffi::c_int),
        785 as ::core::ffi::c_int,
    );
}

extern "C" fn test_end_element_events() {
    set_test_info(b"test_end_element_events\0", 793 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(b"<a><b><c/></b><d><f/></d></a>\0");
    let expected = bytes_as_xml_char_ptr(b"/c/b/f/d/a\0");
    let mut storage = CharData {
        count: 0,
        data: [0; 2048],
    };

    char_data_init(&mut storage);
    parser_set_user_data((&mut storage as *mut CharData).cast());
    parser_set_end_element_handler(end_element_event_handler_for_tests());
    ensure_parser_success(parse_single_bytes_c_string(text), 803 as ::core::ffi::c_int);
    char_data_check_xml_chars(&mut storage, expected);
}
fn is_whitespace_normalized(
    s: &std::ffi::CStr,
    is_cdata: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut blanks: ::core::ffi::c_int = 0;
    let mut at_start = true;

    for &byte in s.to_bytes() {
        if byte == b' ' {
            blanks += 1;
        } else if matches!(byte, b'\t' | b'\n' | b'\r') {
            return 0;
        } else {
            if at_start {
                at_start = false;
                if blanks != 0 && is_cdata == 0 {
                    return 0;
                }
            } else if blanks > 1 && is_cdata == 0 {
                return 0;
            }
            blanks = 0;
        }
    }

    if blanks != 0 && is_cdata == 0 {
        0
    } else {
        1
    }
}
extern "C" fn test_helper_is_whitespace_normalized() {
    set_test_info(
        b"test_helper_is_whitespace_normalized\0",
        848 as ::core::ffi::c_int,
    );

    let cases: [(&[u8], ::core::ffi::c_int, bool, ::core::ffi::c_int, &[u8]); 19] = [
        (
            b"abc\0",
            0,
            true,
            849,
            b"is_whitespace_normalized(XCS(\"abc\"), 0)\0",
        ),
        (
            b"abc\0",
            1,
            true,
            850,
            b"is_whitespace_normalized(XCS(\"abc\"), 1)\0",
        ),
        (
            b"abc def ghi\0",
            0,
            true,
            851,
            b"is_whitespace_normalized(XCS(\"abc def ghi\"), 0)\0",
        ),
        (
            b"abc def ghi\0",
            1,
            true,
            852,
            b"is_whitespace_normalized(XCS(\"abc def ghi\"), 1)\0",
        ),
        (
            b" abc def ghi\0",
            0,
            false,
            853,
            b"! is_whitespace_normalized(XCS(\" abc def ghi\"), 0)\0",
        ),
        (
            b" abc def ghi\0",
            1,
            true,
            854,
            b"is_whitespace_normalized(XCS(\" abc def ghi\"), 1)\0",
        ),
        (
            b"abc  def ghi\0",
            0,
            false,
            855,
            b"! is_whitespace_normalized(XCS(\"abc  def ghi\"), 0)\0",
        ),
        (
            b"abc  def ghi\0",
            1,
            true,
            856,
            b"is_whitespace_normalized(XCS(\"abc  def ghi\"), 1)\0",
        ),
        (
            b"abc def ghi \0",
            0,
            false,
            857,
            b"! is_whitespace_normalized(XCS(\"abc def ghi \"), 0)\0",
        ),
        (
            b"abc def ghi \0",
            1,
            true,
            858,
            b"is_whitespace_normalized(XCS(\"abc def ghi \"), 1)\0",
        ),
        (
            b" \0",
            0,
            false,
            859,
            b"! is_whitespace_normalized(XCS(\" \"), 0)\0",
        ),
        (
            b" \0",
            1,
            true,
            860,
            b"is_whitespace_normalized(XCS(\" \"), 1)\0",
        ),
        (
            b"\t\0",
            0,
            false,
            861,
            b"! is_whitespace_normalized(XCS(\"\\t\"), 0)\0",
        ),
        (
            b"\t\0",
            1,
            false,
            862,
            b"! is_whitespace_normalized(XCS(\"\\t\"), 1)\0",
        ),
        (
            b"\n\0",
            0,
            false,
            863,
            b"! is_whitespace_normalized(XCS(\"\\n\"), 0)\0",
        ),
        (
            b"\n\0",
            1,
            false,
            864,
            b"! is_whitespace_normalized(XCS(\"\\n\"), 1)\0",
        ),
        (
            b"\r\0",
            0,
            false,
            865,
            b"! is_whitespace_normalized(XCS(\"\\r\"), 0)\0",
        ),
        (
            b"\r\0",
            1,
            false,
            866,
            b"! is_whitespace_normalized(XCS(\"\\r\"), 1)\0",
        ),
        (
            b"abc\t def\0",
            1,
            false,
            867,
            b"! is_whitespace_normalized(XCS(\"abc\\t def\"), 1)\0",
        ),
    ];

    for (text, is_cdata, expected, line, message) in cases {
        let actual = is_whitespace_normalized(cstr(text), is_cdata) != 0;
        assert_test_condition(actual == expected, line, message);
    }
}
extern "C" fn check_attr_contains_normalized_whitespace(
    _user_data: *mut ::core::ffi::c_void,
    _name: *const XML_Char,
    atts: *mut *const XML_Char,
) {
    for (attrname, value) in attr_pairs(atts) {
        let tracked_attr = xml_string_equals(attrname, b"attr\0")
            || xml_string_equals(attrname, b"ents\0")
            || xml_string_equals(attrname, b"refs\0");
        if tracked_attr && is_whitespace_normalized(c_str_from_ptr(value.cast()), 0) == 0 {
            let mut buffer: [::core::ffi::c_char; 256] = [0; 256];
            format_attr_normalization_failure(&mut buffer, attrname, value);
            fail_test_with_buffer(889 as ::core::ffi::c_int, buffer.as_mut_ptr());
        }
    }
}
extern "C" fn test_attr_whitespace_normalization() {
    set_test_info(
        b"test_attr_whitespace_normalization\0",
        895 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<!DOCTYPE doc [\n  <!ATTLIST doc\n            attr NMTOKENS #REQUIRED\n            ents ENTITIES #REQUIRED\n            refs IDREFS   #REQUIRED>\n]>\n<doc attr='    a  b c\t\td\te\t' refs=' id-1   \t  id-2\t\t'  \n     ents=' ent-1   \t\r\n            ent-2  ' >\n  <e id='id-1'/>\n  <e id='id-2'/>\n</doc>\0",
    );

    parser_set_start_element_handler(attr_whitespace_handler_for_tests());
    ensure_parser_success(parse_single_bytes_c_string(text), 914 as ::core::ffi::c_int);
}
extern "C" fn test_xmldecl_misplaced() {
    set_test_info(b"test_xmldecl_misplaced\0", 922 as ::core::ffi::c_int);
    expect_failure(
        bytes_as_c_char_ptr(b"\n<?xml version='1.0'?>\n<a/>\0"),
        XML_ERROR_MISPLACED_XML_PI,
        b"failed to report misplaced XML declaration\0",
        927 as ::core::ffi::c_int,
    );
}

extern "C" fn test_xmldecl_invalid() {
    set_test_info(b"test_xmldecl_invalid\0", 931 as ::core::ffi::c_int);
    expect_failure(
        bytes_as_c_char_ptr(b"<?xml version='1.0' \xC3\xA7?>\n<doc/>\0"),
        XML_ERROR_XML_DECL,
        b"Failed to report invalid XML declaration\0",
        933 as ::core::ffi::c_int,
    );
}

extern "C" fn test_xmldecl_missing_attr() {
    set_test_info(b"test_xmldecl_missing_attr\0", 937 as ::core::ffi::c_int);
    expect_failure(
        bytes_as_c_char_ptr(b"<?xml ='1.0'?>\n<doc/>\n\0"),
        XML_ERROR_XML_DECL,
        b"Failed to report missing XML declaration attribute\0",
        939 as ::core::ffi::c_int,
    );
}

extern "C" fn test_xmldecl_missing_value() {
    set_test_info(b"test_xmldecl_missing_value\0", 943 as ::core::ffi::c_int);
    expect_failure(
        bytes_as_c_char_ptr(b"<?xml version='1.0' encoding='us-ascii' standalone?>\n<doc/>\0"),
        XML_ERROR_XML_DECL,
        b"Failed to report missing attribute value\0",
        947 as ::core::ffi::c_int,
    );
}

extern "C" fn test_unknown_encoding_internal_entity() {
    set_test_info(
        b"test_unknown_encoding_internal_entity\0",
        952 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='unsupported-encoding'?>\n<!DOCTYPE test [<!ENTITY foo 'bar'>]>\n<test a='&foo;'/>\0",
    );

    parser_set_unknown_encoding_handler(unknown_encoding_handler_for_tests(), NULL);
    ensure_parser_success(parse_single_bytes_c_string(text), 960 as ::core::ffi::c_int);
}

extern "C" fn test_unrecognised_encoding_internal_entity() {
    set_test_info(
        b"test_unrecognised_encoding_internal_entity\0",
        965 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='unsupported-encoding'?>\n<!DOCTYPE test [<!ENTITY foo 'bar'>]>\n<test a='&foo;'/>\0",
    );

    parser_set_unknown_encoding_handler(unrecognised_encoding_handler_for_tests(), NULL);
    if !parser_status_is_error(parse_single_bytes_c_string(text)) {
        fail_test(
            973 as ::core::ffi::c_int,
            b"Unrecognised encoding not rejected\0",
        );
    }
}

extern "C" fn test_ext_entity_set_encoding() {
    set_test_info(b"test_ext_entity_set_encoding\0", 978 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0",
    );
    let mut test_data = ExtTest {
        parse_text: bytes_as_c_char_ptr(b"<?xml encoding='iso-8859-3'?>\xC3\xA9\0"),
        encoding: bytes_as_xml_char_ptr(b"utf-8\0"),
        storage: ::core::ptr::null_mut::<CharData>(),
    };

    parser_set_external_entity_ref_handler(external_entity_loader_handler_for_tests());
    run_ext_character_check(
        text,
        &mut test_data,
        bytes_as_xml_char_ptr(b"\xC3\xA9\0"),
        995 as ::core::ffi::c_int,
    );
}

extern "C" fn test_ext_entity_no_handler() {
    set_test_info(b"test_ext_entity_no_handler\0", 1000 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0",
    );

    parser_set_default_handler(default_handler());
    run_character_check(
        text,
        bytes_as_xml_char_ptr(b"\0"),
        1007 as ::core::ffi::c_int,
    );
}

extern "C" fn test_ext_entity_set_bom() {
    set_test_info(b"test_ext_entity_set_bom\0", 1012 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0",
    );
    let mut test_data = ExtTest {
        parse_text: bytes_as_c_char_ptr(b"\xEF\xBB\xBF<?xml encoding='iso-8859-3'?>\xC3\xA9\0"),
        encoding: bytes_as_xml_char_ptr(b"utf-8\0"),
        storage: ::core::ptr::null_mut::<CharData>(),
    };

    parser_set_external_entity_ref_handler(external_entity_loader_handler_for_tests());
    run_ext_character_check(
        text,
        &mut test_data,
        bytes_as_xml_char_ptr(b"\xC3\xA9\0"),
        1028 as ::core::ffi::c_int,
    );
}

extern "C" fn test_ext_entity_bad_encoding() {
    set_test_info(
        b"test_ext_entity_bad_encoding\0",
        1033 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0",
    );
    let mut fault = ext_faults {
        parse_text: bytes_as_c_char_ptr(b"<?xml encoding='iso-8859-3'?>u\0"),
        fail_text: bytes_as_c_char_ptr(b"Unsupported encoding not faulted\0"),
        encoding: bytes_as_xml_char_ptr(b"unknown\0"),
        error: XML_ERROR_UNKNOWN_ENCODING,
    };

    parser_set_external_entity_ref_handler(external_entity_faulter_handler_for_tests());
    parser_set_user_data((&mut fault as *mut ExtFaults).cast());
    expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Bad encoding should not have been accepted\0",
        1045 as ::core::ffi::c_int,
    );
}

extern "C" fn test_ext_entity_bad_encoding_2() {
    set_test_info(
        b"test_ext_entity_bad_encoding_2\0",
        1050 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0",
    );
    let mut fault = ext_faults {
        parse_text: bytes_as_c_char_ptr(b"<!ELEMENT doc (#PCDATA)*>\0"),
        fail_text: bytes_as_c_char_ptr(b"Unknown encoding not faulted\0"),
        encoding: bytes_as_xml_char_ptr(b"unknown-encoding\0"),
        error: XML_ERROR_UNKNOWN_ENCODING,
    };

    parser_set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
    parser_set_external_entity_ref_handler(external_entity_faulter_handler_for_tests());
    parser_set_user_data((&mut fault as *mut ExtFaults).cast());
    expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Bad encoding not faulted in external entity handler\0",
        1062 as ::core::ffi::c_int,
    );
}

extern "C" fn test_wfc_undeclared_entity_unread_external_subset() {
    set_test_info(
        b"test_wfc_undeclared_entity_unread_external_subset\0",
        1069 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(b"<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0");

    ensure_parser_success(
        parse_single_bytes_c_string(text),
        1075 as ::core::ffi::c_int,
    );
}

extern "C" fn test_wfc_undeclared_entity_no_external_subset() {
    set_test_info(
        b"test_wfc_undeclared_entity_no_external_subset\0",
        1082 as ::core::ffi::c_int,
    );
    expect_failure(
        bytes_as_c_char_ptr(b"<doc>&entity;</doc>\0"),
        XML_ERROR_UNDEFINED_ENTITY,
        b"Parser did not report undefined entity w/out a DTD.\0",
        1084 as ::core::ffi::c_int,
    );
}

extern "C" fn test_wfc_undeclared_entity_standalone() {
    set_test_info(
        b"test_wfc_undeclared_entity_standalone\0",
        1091 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='us-ascii' standalone='yes'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0",
    );

    expect_failure(
        text,
        XML_ERROR_UNDEFINED_ENTITY,
        b"Parser did not report undefined entity (standalone).\0",
        1098 as ::core::ffi::c_int,
    );
}

extern "C" fn test_wfc_undeclared_entity_with_external_subset_standalone() {
    set_test_info(
        b"test_wfc_undeclared_entity_with_external_subset_standalone\0",
        1105 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='us-ascii' standalone='yes'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0",
    );
    let mut test_data = ExtTest {
        parse_text: bytes_as_c_char_ptr(b"<!ELEMENT doc (#PCDATA)*>\0"),
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };

    parser_set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
    parser_set_user_data((&mut test_data as *mut ExtTest).cast());
    parser_set_external_entity_ref_handler(external_entity_loader_handler_for_tests());
    expect_failure(
        text,
        XML_ERROR_UNDEFINED_ENTITY,
        b"Parser did not report undefined entity (external DTD).\0",
        1116 as ::core::ffi::c_int,
    );
}

extern "C" fn test_entity_with_external_subset_unless_standalone() {
    set_test_info(
        b"test_entity_with_external_subset_unless_standalone\0",
        1123 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='us-ascii' standalone='yes'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0",
    );
    let mut test_data = ExtTest {
        parse_text: bytes_as_c_char_ptr(b"<!ENTITY entity 'bar'>\0"),
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };

    parser_set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE);
    parser_set_user_data((&mut test_data as *mut ExtTest).cast());
    parser_set_external_entity_ref_handler(external_entity_loader_handler_for_tests());
    expect_failure(
        text,
        XML_ERROR_UNDEFINED_ENTITY,
        b"Parser did not report undefined entity\0",
        1135 as ::core::ffi::c_int,
    );
}

extern "C" fn test_wfc_undeclared_entity_with_external_subset() {
    set_test_info(
        b"test_wfc_undeclared_entity_with_external_subset\0",
        1142 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0",
    );
    let mut test_data = ExtTest {
        parse_text: bytes_as_c_char_ptr(b"<!ELEMENT doc (#PCDATA)*>\0"),
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };

    parser_set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
    parser_set_external_entity_ref_handler(external_entity_loader_handler_for_tests());
    run_ext_character_check(
        text,
        &mut test_data,
        bytes_as_xml_char_ptr(b"\0"),
        1150 as ::core::ffi::c_int,
    );
}

extern "C" fn test_not_standalone_handler_reject() {
    set_test_info(
        b"test_not_standalone_handler_reject\0",
        1155 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0",
    );
    let mut test_data = ExtTest {
        parse_text: bytes_as_c_char_ptr(b"<!ELEMENT doc (#PCDATA)*>\0"),
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };

    parser_set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
    parser_set_user_data((&mut test_data as *mut ExtTest).cast());
    parser_set_external_entity_ref_handler(external_entity_loader_handler_for_tests());
    parser_set_not_standalone_handler(reject_not_standalone_handler_for_tests());
    expect_failure(
        text,
        XML_ERROR_NOT_STANDALONE,
        b"NotStandalone handler failed to reject\0",
        1166 as ::core::ffi::c_int,
    );

    parser_reset();
    parser_set_not_standalone_handler(reject_not_standalone_handler_for_tests());
    expect_failure(
        text,
        XML_ERROR_NOT_STANDALONE,
        b"NotStandalone handler failed to reject\0",
        1172 as ::core::ffi::c_int,
    );
}

extern "C" fn test_not_standalone_handler_accept() {
    set_test_info(
        b"test_not_standalone_handler_accept\0",
        1177 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0",
    );
    let mut test_data = ExtTest {
        parse_text: bytes_as_c_char_ptr(b"<!ELEMENT doc (#PCDATA)*>\0"),
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };

    parser_set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
    parser_set_external_entity_ref_handler(external_entity_loader_handler_for_tests());
    parser_set_not_standalone_handler(accept_not_standalone_handler_for_tests());
    run_ext_character_check(
        text,
        &mut test_data,
        bytes_as_xml_char_ptr(b"\0"),
        1186 as ::core::ffi::c_int,
    );

    parser_reset();
    parser_set_not_standalone_handler(accept_not_standalone_handler_for_tests());
    run_character_check(
        text,
        bytes_as_xml_char_ptr(b"\0"),
        1191 as ::core::ffi::c_int,
    );
}
extern "C" fn test_entity_start_tag_level_greater_than_one() {
    set_test_info(
        b"test_entity_start_tag_level_greater_than_one\0",
        1195 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<!DOCTYPE t1 [\n  <!ENTITY e1 'hello'>\n]>\n<t1>\n  <t2>&e1;</t2>\n</t1>\n\0",
    );
    let parser = create_parser_or_fail(1206 as ::core::ffi::c_int);
    let status = parse_single_bytes_c_string_for(parser, text);
    if status as ::core::ffi::c_uint != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        fail_test(
            1206 as ::core::ffi::c_int,
            b"check failed: _XML_Parse_SINGLE_BYTES(parser, text, (int)strlen(text), XML_TRUE) == XML_STATUS_OK\0",
        );
    }
    parser_free(parser);
}
extern "C" fn test_wfc_no_recursive_entity_refs() {
    set_test_info(
        b"test_wfc_no_recursive_entity_refs\0",
        1211 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<!DOCTYPE doc [\n  <!ENTITY entity '&#38;entity;'>\n]>\n<doc>&entity;</doc>\0",
    );
    expect_failure(
        text,
        XML_ERROR_RECURSIVE_ENTITY_REF,
        b"Parser did not report recursive entity reference.\0",
        1218 as ::core::ffi::c_int,
    );
}
extern "C" fn test_no_indirectly_recursive_entity_refs() {
    set_test_info(
        b"test_no_indirectly_recursive_entity_refs\0",
        1222 as ::core::ffi::c_int,
    );
    let cases = [
        TestCase_0 {
            doc: bytes_as_c_char_ptr(
                b"<!DOCTYPE a [\n  <!ENTITY e1 '&e2;'>\n  <!ENTITY e2 '&e1;'>\n]><a>&e2;</a>\n\0",
            ),
            usesParameterEntities: false_0 != 0,
        },
        TestCase_0 {
            doc: bytes_as_c_char_ptr(
                b"<!DOCTYPE a [\n  <!ENTITY e1 '&e2;'>\n  <!ENTITY e2 '&e1;'>\n]><a k1='&e2;' />\n\0",
            ),
            usesParameterEntities: false_0 != 0,
        },
        TestCase_0 {
            doc: bytes_as_c_char_ptr(
                b"<!DOCTYPE doc [\n  <!ENTITY % p1 '&#37;p2;'>\n  <!ENTITY % p2 '&#37;p1;'>\n  <!ENTITY % define_g \"<!ENTITY g '&#37;p2;'>\">\n  %define_g;\n]>\n<doc/>\n\0",
            ),
            usesParameterEntities: true_0 != 0,
        },
    ];
    let reset_or_not = [XML_TRUE, XML_FALSE];

    for (i, case) in cases.iter().enumerate() {
        for (j, &reset_wanted) in reset_or_not.iter().enumerate() {
            set_subtest_message(&format!(
                "[{i},reset={j}] {}",
                c_str_from_ptr(case.doc).to_string_lossy()
            ));

            let parser = create_parser_or_fail(1278 as ::core::ffi::c_int);
            if case.usesParameterEntities
                && parser_set_param_entity_parsing_for(parser, XML_PARAM_ENTITY_PARSING_ALWAYS)
                    != 1 as ::core::ffi::c_int
            {
                fail_test(
                    1278 as ::core::ffi::c_int,
                    b"check failed: XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS) == 1\0",
                );
            }

            let status = parse_single_bytes_c_string_for(parser, case.doc);
            if status as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                fail_test(
                    1289 as ::core::ffi::c_int,
                    b"check failed: status == XML_STATUS_ERROR\0",
                );
            }
            if parser_error_code_for(parser) as ::core::ffi::c_uint
                != XML_ERROR_RECURSIVE_ENTITY_REF as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                fail_test(
                    1290 as ::core::ffi::c_int,
                    b"check failed: XML_GetErrorCode(parser) == XML_ERROR_RECURSIVE_ENTITY_REF\0",
                );
            }

            if reset_wanted != 0 {
                parser_reset_for(parser);
            }
            parser_free(parser);
        }
    }
}
extern "C" fn test_recursive_external_parameter_entity_2() {
    set_test_info(
        b"test_recursive_external_parameter_entity_2\0",
        1309 as ::core::ffi::c_int,
    );
    let cases = [
        TestCase {
            doc: bytes_as_c_char_ptr(b"<!ENTITY % p1 '%p1;'>\0"),
            expectedStatus: XML_STATUS_ERROR,
        },
        TestCase {
            doc: bytes_as_c_char_ptr(
                b"<!ENTITY % p1 '%p1;'><!ENTITY % p1 'first declaration wins'>\0",
            ),
            expectedStatus: XML_STATUS_ERROR,
        },
        TestCase {
            doc: bytes_as_c_char_ptr(
                b"<!ENTITY % p1 'first declaration wins'><!ENTITY % p1 '%p1;'>\0",
            ),
            expectedStatus: XML_STATUS_OK,
        },
        TestCase {
            doc: bytes_as_c_char_ptr(b"<!ENTITY % p1 '&#37;p1;'>\0"),
            expectedStatus: XML_STATUS_OK,
        },
    ];

    for case in cases {
        set_subtest_message(&c_str_from_ptr(case.doc).to_string_lossy());
        let parser = create_parser_or_fail(1332 as ::core::ffi::c_int);
        let ext_parser = create_external_entity_parser_or_fail(parser, 1335 as ::core::ffi::c_int);
        let actual_status = parse_single_bytes_c_string_for(ext_parser, case.doc);

        if actual_status as ::core::ffi::c_uint != case.expectedStatus as ::core::ffi::c_uint {
            fail_test(
                1340 as ::core::ffi::c_int,
                b"check failed: actualStatus == expectedStatus\0",
            );
        }
        if actual_status as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            && parser_error_code_for(ext_parser) as ::core::ffi::c_uint
                != XML_ERROR_RECURSIVE_ENTITY_REF as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            fail_test(
                1343 as ::core::ffi::c_int,
                b"check failed: XML_GetErrorCode(ext_parser) == XML_ERROR_RECURSIVE_ENTITY_REF\0",
            );
        }

        parser_free(ext_parser);
        parser_free(parser);
    }
}
extern "C" fn test_ext_entity_invalid_parse() {
    set_test_info(
        b"test_ext_entity_invalid_parse\0",
        1353 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0",
    );
    let faults = [
        ext_faults {
            parse_text: bytes_as_c_char_ptr(b"<\0"),
            fail_text: bytes_as_c_char_ptr(b"Incomplete element declaration not faulted\0"),
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_UNCLOSED_TOKEN,
        },
        ext_faults {
            parse_text: bytes_as_c_char_ptr(b"<\xE2\x82\0"),
            fail_text: bytes_as_c_char_ptr(b"Incomplete character not faulted\0"),
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_PARTIAL_CHAR,
        },
        ext_faults {
            parse_text: bytes_as_c_char_ptr(b"<tag>\xE2\x82\0"),
            fail_text: bytes_as_c_char_ptr(b"Incomplete character in CDATA not faulted\0"),
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_PARTIAL_CHAR,
        },
    ];

    for fault in faults.iter() {
        set_subtest_message(&format!(
            "\"{}\"",
            c_str_from_ptr(fault.parse_text).to_string_lossy()
        ));
        parser_set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
        parser_set_external_entity_ref_handler(Some(
            external_entity_faulter
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ));
        parser_set_user_data(fault as *const ExtFaults as *mut ::core::ffi::c_void);
        expect_failure(
            text,
            XML_ERROR_EXTERNAL_ENTITY_HANDLING,
            b"Parser did not report external entity error\0",
            1374 as ::core::ffi::c_int,
        );
        parser_reset();
    }
}
extern "C" fn test_dtd_default_handling() {
    unsafe {
        _check_set_test_info(
            b"test_dtd_default_handling\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1381 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ENTITY e SYSTEM 'http://example.org/e'>\n<!NOTATION n SYSTEM 'http://example.org/n'>\n<!ELEMENT doc EMPTY>\n<!ATTLIST doc a CDATA #IMPLIED>\n<?pi in dtd?>\n<!--comment in dtd-->\n]><doc/>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        XML_SetDefaultHandler(
            g_parser,
            Some(
                accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetStartDoctypeDeclHandler(
            g_parser,
            Some(
                dummy_start_doctype_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetEndDoctypeDeclHandler(
            g_parser,
            Some(dummy_end_doctype_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        XML_SetEntityDeclHandler(
            g_parser,
            Some(
                dummy_entity_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                        *const XML_Char,
                        ::core::ffi::c_int,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetNotationDeclHandler(
            g_parser,
            Some(
                dummy_notation_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetElementDeclHandler(
            g_parser,
            Some(
                dummy_element_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut XML_Content,
                    ) -> (),
            ),
        );
        XML_SetAttlistDeclHandler(
            g_parser,
            Some(
                dummy_attlist_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetProcessingInstructionHandler(
            g_parser,
            Some(
                dummy_pi_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetCommentHandler(
            g_parser,
            Some(
                dummy_comment_handler
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
            ),
        );
        XML_SetStartCdataSectionHandler(
            g_parser,
            Some(dummy_start_cdata_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        XML_SetEndCdataSectionHandler(
            g_parser,
            Some(dummy_end_cdata_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        _run_character_check(
            text,
            b"\n\n\n\n\n\n\n<doc/>\0".as_ptr() as *const XML_Char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1402 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_dtd_attr_handling() {
    unsafe {
        _check_set_test_info(
            b"test_dtd_attr_handling\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1407 as ::core::ffi::c_int,
        );
        let mut prolog: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [\n<!ELEMENT doc EMPTY>\n\0".as_ptr() as *const ::core::ffi::c_char;
        let mut attr_data: [AttTest; 5] = [
            AttTest {
                definition: b"<!ATTLIST doc a ( one | two | three ) #REQUIRED>\n]><doc a='two'/>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                element_name: b"doc\0".as_ptr() as *const XML_Char,
                attr_name: b"a\0".as_ptr() as *const XML_Char,
                attr_type: b"(one|two|three)\0".as_ptr() as *const XML_Char,
                default_value: ::core::ptr::null::<XML_Char>(),
                is_required: XML_TRUE as ::core::ffi::c_int,
            },
            AttTest {
                definition: b"<!NOTATION foo SYSTEM 'http://example.org/foo'>\n<!ATTLIST doc a NOTATION (foo) #IMPLIED>\n]><doc/>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                element_name: b"doc\0".as_ptr() as *const XML_Char,
                attr_name: b"a\0".as_ptr() as *const XML_Char,
                attr_type: b"NOTATION(foo)\0".as_ptr() as *const XML_Char,
                default_value: ::core::ptr::null::<XML_Char>(),
                is_required: XML_FALSE as ::core::ffi::c_int,
            },
            AttTest {
                definition: b"<!ATTLIST doc a NOTATION (foo) 'bar'>\n]><doc/>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                element_name: b"doc\0".as_ptr() as *const XML_Char,
                attr_name: b"a\0".as_ptr() as *const XML_Char,
                attr_type: b"NOTATION(foo)\0".as_ptr() as *const XML_Char,
                default_value: b"bar\0".as_ptr() as *const XML_Char,
                is_required: XML_FALSE as ::core::ffi::c_int,
            },
            AttTest {
                definition: b"<!ATTLIST doc a CDATA '\xDB\xB2'>\n]><doc/>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                element_name: b"doc\0".as_ptr() as *const XML_Char,
                attr_name: b"a\0".as_ptr() as *const XML_Char,
                attr_type: b"CDATA\0".as_ptr() as *const XML_Char,
                default_value: b"\xDB\xB2\0".as_ptr() as *const XML_Char,
                is_required: XML_FALSE as ::core::ffi::c_int,
            },
            AttTest {
                definition: ::core::ptr::null::<::core::ffi::c_char>(),
                element_name: ::core::ptr::null::<XML_Char>(),
                attr_name: ::core::ptr::null::<XML_Char>(),
                attr_type: ::core::ptr::null::<XML_Char>(),
                default_value: ::core::ptr::null::<XML_Char>(),
                is_required: XML_FALSE as ::core::ffi::c_int,
            },
        ];
        let mut test: *mut AttTest = ::core::ptr::null_mut::<AttTest>();
        test = &raw mut attr_data as *mut AttTest;
        while !(*test).definition.is_null() {
            set_subtest(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                (*test).definition,
            );
            XML_SetAttlistDeclHandler(
                g_parser,
                Some(
                    verify_attlist_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
            );
            XML_SetUserData(g_parser, test as *mut ::core::ffi::c_void);
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                prolog,
                strlen(prolog) as ::core::ffi::c_int,
                XML_FALSE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _xml_failure(
                    g_parser,
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1446 as ::core::ffi::c_int,
                );
            }
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                (*test).definition,
                strlen((*test).definition) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _xml_failure(
                    g_parser,
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1450 as ::core::ffi::c_int,
                );
            }
            XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
            test = test.offset(1);
        }
    }
}
extern "C" fn test_empty_ns_without_namespaces() {
    unsafe {
        _check_set_test_info(
            b"test_empty_ns_without_namespaces\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1462 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc xmlns:prefix='http://example.org/'>\n  <e xmlns:prefix=''/>\n</doc>\0".as_ptr()
                as *const ::core::ffi::c_char;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1469 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_ns_in_attribute_default_without_namespaces() {
    unsafe {
        _check_set_test_info(
            b"test_ns_in_attribute_default_without_namespaces\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1477 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE e:element [\n  <!ATTLIST e:element\n    xmlns:e CDATA 'http://example.org/'>\n      ]>\n<e:element/>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1486 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_stop_parser_between_char_data_calls() {
    set_test_info(
        b"test_stop_parser_between_char_data_calls\0",
        1492 as ::core::ffi::c_int,
    );
    let text = shared_test_text(SharedTestText::CharacterData);
    parser_set_character_data_handler(clearing_aborting_character_data_handler());
    set_parser_stop_state(XML_FALSE, None);
    if parse_single_bytes_c_string(text) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(1505 as ::core::ffi::c_int);
    }
    if parser_error_code() as ::core::ffi::c_uint
        != XML_ERROR_ABORTED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(1507 as ::core::ffi::c_int);
    }
}
extern "C" fn test_suspend_parser_between_char_data_calls() {
    set_test_info(
        b"test_suspend_parser_between_char_data_calls\0",
        1513 as ::core::ffi::c_int,
    );
    let text = shared_test_text(SharedTestText::CharacterData);
    parser_set_character_data_handler(clearing_aborting_character_data_handler());
    set_parser_stop_state(XML_TRUE, None);
    if parser_parse_c_string(text) as ::core::ffi::c_uint
        != XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(1528 as ::core::ffi::c_int);
    }
    if parser_error_code() as ::core::ffi::c_uint
        != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(1530 as ::core::ffi::c_int);
    }
    if parse_single_bytes_c_string(text) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            1534 as ::core::ffi::c_int,
            b"Attempt to continue parse while suspended not faulted\0",
        );
    }
    if parser_error_code() as ::core::ffi::c_uint
        != XML_ERROR_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            1536 as ::core::ffi::c_int,
            b"Suspended parse not faulted with correct error\0",
        );
    }
}
extern "C" fn test_repeated_stop_parser_between_char_data_calls() {
    set_test_info(
        b"test_repeated_stop_parser_between_char_data_calls\0",
        1541 as ::core::ffi::c_int,
    );
    let text = shared_test_text(SharedTestText::CharacterData);

    parser_set_character_data_handler(parser_stop_character_data_handler());
    set_parser_stop_state(XML_FALSE, Some(XML_FALSE));
    if parse_single_bytes_c_string(text) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            1549 as ::core::ffi::c_int,
            b"Failed to double-stop parser\0",
        );
    }

    parser_reset();
    parser_set_character_data_handler(parser_stop_character_data_handler());
    set_parser_stop_state(XML_TRUE, Some(XML_FALSE));
    if parser_parse_c_string(text) as ::core::ffi::c_uint
        != XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            1559 as ::core::ffi::c_int,
            b"Failed to double-suspend parser\0",
        );
    }

    parser_reset();
    parser_set_character_data_handler(parser_stop_character_data_handler());
    set_parser_stop_state(XML_TRUE, Some(XML_TRUE));
    if parse_single_bytes_c_string(text) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            1567 as ::core::ffi::c_int,
            b"Failed to suspend-abort parser\0",
        );
    }
}
extern "C" fn test_good_cdata_ascii() {
    set_test_info(b"test_good_cdata_ascii\0", 1571 as ::core::ffi::c_int);
    let text = b"<a><![CDATA[<greeting>Hello, world!</greeting>]]></a>\0";
    let expected = bytes_as_xml_char_ptr(b"<greeting>Hello, world!</greeting>\0");
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };

    char_data_init(&mut storage);
    parser_set_user_data((&raw mut storage).cast());
    parser_set_character_data_handler(accumulating_character_handler());
    parser_set_start_cdata_section_handler(start_cdata_handler_for_tests());
    parser_set_end_cdata_section_handler(end_cdata_handler_for_tests());
    ensure_parser_success(
        parse_single_bytes_buffer(text, XML_TRUE as ::core::ffi::c_int),
        1585 as ::core::ffi::c_int,
    );
    char_data_check_xml_chars(&mut storage, expected);

    parser_reset();
    char_data_init(&mut storage);
    parser_set_user_data((&raw mut storage).cast());
    parser_set_character_data_handler(accumulating_character_handler());
    parser_set_default_handler(default_handler());
    ensure_parser_success(
        parse_single_bytes_buffer(text, XML_TRUE as ::core::ffi::c_int),
        1597 as ::core::ffi::c_int,
    );
    char_data_check_xml_chars(&mut storage, expected);
}
extern "C" fn test_good_cdata_utf16() {
    set_test_info(b"test_good_cdata_utf16\0", 1602 as ::core::ffi::c_int);
    let text =
        b"\0<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\x001\0.\x000\0'\0 \0e\0n\0c\0o\0d\0i\0n\0g\0=\0'\0u\0t\0f\0-\x001\x006\0'\0?\0>\0\n\0<\0a\0>\0<\0!\0[\0C\0D\0A\0T\0A\0[\0h\0e\0l\0l\0o\0]\0]\0>\0<\0/\0a\0>\0";
    let expected = bytes_as_xml_char_ptr(b"hello\0");

    expect_character_data_from_single_bytes(text, expected, 1624 as ::core::ffi::c_int);
}
extern "C" fn test_good_cdata_utf16_le() {
    set_test_info(b"test_good_cdata_utf16_le\0", 1629 as ::core::ffi::c_int);
    let text =
        b"<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\x001\0.\x000\0'\0 \0e\0n\0c\0o\0d\0i\0n\0g\0=\0'\0u\0t\0f\0-\x001\x006\0'\0?\0>\0\n\0<\0a\0>\0<\0!\0[\0C\0D\0A\0T\0A\0[\0h\0e\0l\0l\0o\0]\0]\0>\0<\0/\0a\0>\0\0";
    let expected = bytes_as_xml_char_ptr(b"hello\0");
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };

    char_data_init(&mut storage);
    parser_set_user_data((&raw mut storage).cast());
    parser_set_character_data_handler(accumulating_character_handler());
    ensure_parser_success(
        parse_single_bytes_buffer(text, XML_TRUE as ::core::ffi::c_int),
        1651 as ::core::ffi::c_int,
    );
    char_data_check_xml_chars(&mut storage, expected);
}
extern "C" fn test_long_cdata_utf16() {
    unsafe {
        _check_set_test_info(
            b"test_long_cdata_utf16\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1661 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 2197] = ::core::mem::transmute::<
            [u8; 2197],
            [::core::ffi::c_char; 2197],
        >(
            *b"\0<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\x001\0.\x000\0'\0 \0e\0n\0c\0o\0d\0i\0n\0g\0=\0'\0u\0t\0f\0-\x001\x006\0'\0?\0>\0<\0a\0>\0<\0!\0[\0C\0D\0A\0T\0A\0[\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0]\0]\0>\0<\0/\0a\0>\0",
        );
        let mut expected: *const XML_Char = b"ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\0"
            .as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        let mut buffer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        CharData_Init(&raw mut storage);
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        buffer = XML_GetBuffer(
            g_parser,
            (::core::mem::size_of::<[::core::ffi::c_char; 2197]>() as usize)
                .wrapping_sub(1 as usize) as ::core::ffi::c_int,
        );
        if buffer.is_null() {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1722 as ::core::ffi::c_int,
                b"Could not allocate parse buffer\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !buffer.is_null() {
        } else {
            __assert_fail(
                b"buffer != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1723 as ::core::ffi::c_uint,
                b"void test_long_cdata_utf16(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        memcpy(
            buffer,
            &raw const text as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            (::core::mem::size_of::<[::core::ffi::c_char; 2197]>() as size_t)
                .wrapping_sub(1 as size_t),
        );
        if XML_ParseBuffer(
            g_parser,
            (::core::mem::size_of::<[::core::ffi::c_char; 2197]>() as usize)
                .wrapping_sub(1 as usize) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1726 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_multichar_cdata_utf16() {
    set_test_info(b"test_multichar_cdata_utf16\0", 1732 as ::core::ffi::c_int);
    let text =
        b"\0<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\x001\0.\x000\0'\0 \0e\0n\0c\0o\0d\0i\0n\0g\0=\0'\0u\0t\0f\0-\x001\x006\0'\0?\0>\0\n\0<\0a\0>\0<\0!\0[\0C\0D\0A\0T\0A\0[\xD84\xDD^\xD84\xDD_\0]\0]\0>\0<\0/\0a\0>\0";
    let expected = bytes_as_xml_char_ptr(b"\xF0\x9D\x85\x9E\xF0\x9D\x85\x9F\0");

    expect_character_data_from_single_bytes(text, expected, 1766 as ::core::ffi::c_int);
}
extern "C" fn test_utf16_bad_surrogate_pair() {
    set_test_info(
        b"test_utf16_bad_surrogate_pair\0",
        1772 as ::core::ffi::c_int,
    );
    let text =
        b"\0<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\x001\0.\x000\0'\0 \0e\0n\0c\0o\0d\0i\0n\0g\0=\0'\0u\0t\0f\0-\x001\x006\0'\0?\0>\0\n\0<\0a\0>\0<\0!\0[\0C\0D\0A\0T\0A\0[\xDC\0\xD8\0\0]\0]\0>\0<\0/\0a\0>\0";

    if parse_single_bytes_buffer(text, XML_TRUE as ::core::ffi::c_int) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            1793 as ::core::ffi::c_int,
            b"Reversed UTF-16 surrogate pair not faulted\0",
        );
    }

    if parser_error_code() as ::core::ffi::c_uint
        != XML_ERROR_INVALID_TOKEN as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(1795 as ::core::ffi::c_int);
    }
}
extern "C" fn test_bad_cdata() {
    set_test_info(b"test_bad_cdata\0", 1799 as ::core::ffi::c_int);
    let cases = [
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><\0"),
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><!\0"),
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![\0"),
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![C\0"),
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![CD\0"),
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![CDA\0"),
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![CDAT\0"),
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![CDATA\0"),
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![CDATA[\0"),
            expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![CDATA[]\0"),
            expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![CDATA[]]\0"),
            expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><!<a/>\0"),
            expectedError: XML_ERROR_INVALID_TOKEN,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![<a/>\0"),
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![C<a/>\0"),
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![CD<a/>\0"),
            expectedError: XML_ERROR_INVALID_TOKEN,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![CDA<a/>\0"),
            expectedError: XML_ERROR_INVALID_TOKEN,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![CDAT<a/>\0"),
            expectedError: XML_ERROR_INVALID_TOKEN,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![CDATA<a/>\0"),
            expectedError: XML_ERROR_INVALID_TOKEN,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![CDATA[<a/>\0"),
            expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![CDATA[]<a/>\0"),
            expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData_0 {
            text: bytes_as_c_char_ptr(b"<a><![CDATA[]]<a/>\0"),
            expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
    ];

    for (index, case_data) in cases.iter().enumerate() {
        set_subtest_text(case_data.text);
        let actual_status = parse_single_bytes_c_string(case_data.text);
        ensure_parser_error(actual_status, 1838 as ::core::ffi::c_int);

        let actual_error = parser_error_code();
        if actual_error as ::core::ffi::c_uint != case_data.expectedError as ::core::ffi::c_uint {
            fail_test_message(
                1846 as ::core::ffi::c_int,
                format!(
                    "Expected error {} but got error {} for case {}: \"{}\"",
                    case_data.expectedError as ::core::ffi::c_uint,
                    actual_error as ::core::ffi::c_uint,
                    index + 1,
                    c_string_lossy(case_data.text),
                ),
            );
        }

        parser_reset();
    }
}
extern "C" fn test_bad_cdata_utf16() {
    set_test_info(b"test_bad_cdata_utf16\0", 1855 as ::core::ffi::c_int);
    let prolog =
        b"\0<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\x001\0.\x000\0'\0 \0e\0n\0c\0o\0d\0i\0n\0g\0=\0'\0u\0t\0f\0-\x001\x006\0'\0?\0>\0\n\0<\0a\0>\0";
    let cases = [
        CaseData {
            text_bytes: 1 as size_t,
            text: b"\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 2 as size_t,
            text: b"\0<\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 3 as size_t,
            text: b"\0<\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 4 as size_t,
            text: b"\0<\0!\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 5 as size_t,
            text: b"\0<\0!\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 6 as size_t,
            text: b"\0<\0!\0[\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 7 as size_t,
            text: b"\0<\0!\0[\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 8 as size_t,
            text: b"\0<\0!\0[\0C\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 9 as size_t,
            text: b"\0<\0!\0[\0C\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 10 as size_t,
            text: b"\0<\0!\0[\0C\0D\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 11 as size_t,
            text: b"\0<\0!\0[\0C\0D\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 12 as size_t,
            text: b"\0<\0!\0[\0C\0D\0A\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 13 as size_t,
            text: b"\0<\0!\0[\0C\0D\0A\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 14 as size_t,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 15 as size_t,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 16 as size_t,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 17 as size_t,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 18 as size_t,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0[\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData {
            text_bytes: 19 as size_t,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0[\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData {
            text_bytes: 20 as size_t,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0[\0Z\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData {
            text_bytes: 21 as size_t,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0[\0Z\xD8\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData {
            text_bytes: 22 as size_t,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0[\0Z\xD84\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_PARTIAL_CHAR,
        },
        CaseData {
            text_bytes: 23 as size_t,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0[\0Z\xD84\xDD\0".as_ptr()
                as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_PARTIAL_CHAR,
        },
        CaseData {
            text_bytes: 24 as size_t,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0[\0Z\xD84\xDD^\0".as_ptr()
                as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
    ];

    for (index, case_data) in cases.iter().enumerate() {
        set_subtest_case_number(index + 1);
        ensure_parser_success(
            parse_single_bytes_buffer(prolog, XML_FALSE as ::core::ffi::c_int),
            1908 as ::core::ffi::c_int,
        );

        let actual_status = parse_single_bytes_with_final(
            case_data.text,
            case_data.text_bytes as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        );
        ensure_parser_error(actual_status, 1911 as ::core::ffi::c_int);

        let actual_error = parser_error_code();
        if actual_error as ::core::ffi::c_uint != case_data.expected_error as ::core::ffi::c_uint {
            fail_test_message(
                1922 as ::core::ffi::c_int,
                format!(
                    "Expected error {} ({}), got {} ({}) for case {}",
                    case_data.expected_error as ::core::ffi::c_uint,
                    xml_error_string_lossy(case_data.expected_error),
                    actual_error as ::core::ffi::c_uint,
                    xml_error_string_lossy(actual_error),
                    index + 1,
                ),
            );
        }

        parser_reset();
    }
}
extern "C" fn test_stop_parser_between_cdata_calls() {
    set_test_info(
        b"test_stop_parser_between_cdata_calls\0",
        1930 as ::core::ffi::c_int,
    );
    let text = shared_test_text(SharedTestText::Cdata);
    parser_set_character_data_handler(clearing_aborting_character_data_handler());
    set_parser_stop_state(XML_FALSE, None);
    expect_failure(
        text,
        XML_ERROR_ABORTED,
        b"Parse not aborted in CDATA handler\0",
        1935 as ::core::ffi::c_int,
    );
}
extern "C" fn test_suspend_parser_between_cdata_calls() {
    set_test_info(
        b"test_suspend_parser_between_cdata_calls\0",
        1940 as ::core::ffi::c_int,
    );
    if current_chunk_size() != 0 as ::core::ffi::c_int {
        return;
    }
    let text = shared_test_text(SharedTestText::Cdata);
    let result: XML_Status;
    parser_set_character_data_handler(clearing_aborting_character_data_handler());
    set_parser_stop_state(XML_TRUE, None);
    result = parser_parse_c_string(text);
    if result as ::core::ffi::c_uint
        != XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if result as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            xml_failure(1956 as ::core::ffi::c_int);
        }
        fail_test(
            1957 as ::core::ffi::c_int,
            b"Parse not suspended in CDATA handler\0",
        );
    }
    if parser_error_code() as ::core::ffi::c_uint
        != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(1960 as ::core::ffi::c_int);
    }
}
extern "C" fn test_memory_allocation() {
    unsafe {
        _check_set_test_info(
            b"test_memory_allocation\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1965 as ::core::ffi::c_int,
        );
        let mut buffer: *mut ::core::ffi::c_char =
            XML_MemMalloc(g_parser, 256 as size_t) as *mut ::core::ffi::c_char;
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if buffer.is_null() {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1970 as ::core::ffi::c_int,
                b"Allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            *buffer.offset(0 as ::core::ffi::c_int as isize) = 'T' as i32 as ::core::ffi::c_char;
            *buffer.offset(1 as ::core::ffi::c_int as isize) = 'E' as i32 as ::core::ffi::c_char;
            *buffer.offset(2 as ::core::ffi::c_int as isize) = 'S' as i32 as ::core::ffi::c_char;
            *buffer.offset(3 as ::core::ffi::c_int as isize) = 'T' as i32 as ::core::ffi::c_char;
            *buffer.offset(4 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
            if strcmp(buffer, b"TEST\0".as_ptr() as *const ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
            {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1979 as ::core::ffi::c_int,
                    b"Memory not writable\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                p = XML_MemRealloc(g_parser, buffer as *mut ::core::ffi::c_void, 512 as size_t)
                    as *mut ::core::ffi::c_char;
                if p.is_null() {
                    _fail(
                        b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1983 as ::core::ffi::c_int,
                        b"Reallocation failed\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    buffer = p;
                    *buffer.offset(0 as ::core::ffi::c_int as isize) =
                        'V' as i32 as ::core::ffi::c_char;
                    if strcmp(buffer, b"VEST\0".as_ptr() as *const ::core::ffi::c_char)
                        != 0 as ::core::ffi::c_int
                    {
                        _fail(
                            b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1989 as ::core::ffi::c_int,
                            b"Reallocated memory not writable\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                }
            }
            XML_MemFree(g_parser, buffer as *mut ::core::ffi::c_void);
        };
    }
}
extern "C" fn test_default_current() {
    unsafe {
        _check_set_test_info(
            b"test_default_current\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1999 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc>hell]</doc>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut entity_text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [\n<!ENTITY entity '&#37;'>\n]>\n<doc>&entity;</doc>\0".as_ptr()
                as *const ::core::ffi::c_char;
        set_subtest(b"with defaulting\0".as_ptr() as *const ::core::ffi::c_char);
        let mut storage: handler_record_list = handler_record_list {
            count: 0,
            entries: [handler_record_entry {
                name: ::core::ptr::null::<::core::ffi::c_char>(),
                arg: 0,
            }; 50],
        };
        storage.count = 0 as ::core::ffi::c_int;
        XML_SetDefaultHandler(
            g_parser,
            Some(
                record_default_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                record_cdata_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2015 as ::core::ffi::c_int,
            );
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let c2rust_fresh2 = i;
        i = i + 1;
        let mut e: *const handler_record_entry = _handler_record_get(
            &raw mut storage,
            c2rust_fresh2,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2017 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2017 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e).arg == 5 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2017 as ::core::ffi::c_int,
                b"check failed: e->arg == (5)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut cdata_len_remaining: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
        while cdata_len_remaining > 0 as ::core::ffi::c_int {
            let c2rust_fresh3 = i;
            i = i + 1;
            let mut c_entry: *const handler_record_entry = _handler_record_get(
                &raw mut storage,
                c2rust_fresh3,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2022 as ::core::ffi::c_int,
            );
            if !(strcmp(
                (*c_entry).name,
                b"record_cdata_handler\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int)
            {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2023 as ::core::ffi::c_int,
                    b"check failed: strcmp(c_entry->name, \"record_cdata_handler\") == 0\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            if !((*c_entry).arg > 0 as ::core::ffi::c_int) {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2024 as ::core::ffi::c_int,
                    b"check failed: c_entry->arg > 0\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if !((*c_entry).arg <= cdata_len_remaining) {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2025 as ::core::ffi::c_int,
                    b"check failed: c_entry->arg <= cdata_len_remaining\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            cdata_len_remaining -= (*c_entry).arg;
            let c2rust_fresh4 = i;
            i = i + 1;
            let mut e_0: *const handler_record_entry = _handler_record_get(
                &raw mut storage,
                c2rust_fresh4,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2029 as ::core::ffi::c_int,
            );
            if !(strcmp(
                (*e_0).name,
                b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int)
            {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2029 as ::core::ffi::c_int,
                    b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            if !((*e_0).arg == (*c_entry).arg) {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2029 as ::core::ffi::c_int,
                    b"check failed: e->arg == (c_entry->arg)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        }
        let c2rust_fresh5 = i;
        i = i + 1;
        let mut e_1: *const handler_record_entry = _handler_record_get(
            &raw mut storage,
            c2rust_fresh5,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2031 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_1).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2031 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_1).arg == 6 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2031 as ::core::ffi::c_int,
                b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !(storage.count == i) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2032 as ::core::ffi::c_int,
                b"check failed: storage.count == i\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        set_subtest(b"no defaulting\0".as_ptr() as *const ::core::ffi::c_char);
        let mut storage_0: handler_record_list = handler_record_list {
            count: 0,
            entries: [handler_record_entry {
                name: ::core::ptr::null::<::core::ffi::c_char>(),
                arg: 0,
            }; 50],
        };
        storage_0.count = 0 as ::core::ffi::c_int;
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        XML_SetDefaultHandler(
            g_parser,
            Some(
                record_default_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                record_cdata_nodefault_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage_0 as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2046 as ::core::ffi::c_int,
            );
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let c2rust_fresh6 = i_0;
        i_0 = i_0 + 1;
        let mut e_2: *const handler_record_entry = _handler_record_get(
            &raw mut storage_0,
            c2rust_fresh6,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2048 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_2).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2048 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_2).arg == 5 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2048 as ::core::ffi::c_int,
                b"check failed: e->arg == (5)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut cdata_len_remaining_0: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
        while cdata_len_remaining_0 > 0 as ::core::ffi::c_int {
            let c2rust_fresh7 = i_0;
            i_0 = i_0 + 1;
            let mut c_entry_0: *const handler_record_entry = _handler_record_get(
                &raw mut storage_0,
                c2rust_fresh7,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2053 as ::core::ffi::c_int,
            );
            if !(strcmp(
                (*c_entry_0).name,
                b"record_cdata_nodefault_handler\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int)
            {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2054 as ::core::ffi::c_int,
                    b"check failed: strcmp(c_entry->name, \"record_cdata_nodefault_handler\") == 0\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if !((*c_entry_0).arg > 0 as ::core::ffi::c_int) {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2055 as ::core::ffi::c_int,
                    b"check failed: c_entry->arg > 0\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if !((*c_entry_0).arg <= cdata_len_remaining_0) {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2056 as ::core::ffi::c_int,
                    b"check failed: c_entry->arg <= cdata_len_remaining\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            cdata_len_remaining_0 -= (*c_entry_0).arg;
        }
        let c2rust_fresh8 = i_0;
        i_0 = i_0 + 1;
        let mut e_3: *const handler_record_entry = _handler_record_get(
            &raw mut storage_0,
            c2rust_fresh8,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2059 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_3).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2059 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_3).arg == 6 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2059 as ::core::ffi::c_int,
                b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !(storage_0.count == i_0) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2060 as ::core::ffi::c_int,
                b"check failed: storage.count == i\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        set_subtest(b"with internal entity\0".as_ptr() as *const ::core::ffi::c_char);
        let mut storage_1: handler_record_list = handler_record_list {
            count: 0,
            entries: [handler_record_entry {
                name: ::core::ptr::null::<::core::ffi::c_char>(),
                arg: 0,
            }; 50],
        };
        storage_1.count = 0 as ::core::ffi::c_int;
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        XML_SetDefaultHandler(
            g_parser,
            Some(
                record_default_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                record_cdata_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage_1 as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            entity_text,
            strlen(entity_text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2075 as ::core::ffi::c_int,
            );
        }
        let mut e_4: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            0 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2077 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_4).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2077 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_4).arg == 9 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2077 as ::core::ffi::c_int,
                b"check failed: e->arg == (9)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_5: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            1 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2078 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_5).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2078 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_5).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2078 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_6: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            2 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2079 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_6).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2079 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_6).arg == 3 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2079 as ::core::ffi::c_int,
                b"check failed: e->arg == (3)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_7: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            3 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2080 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_7).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2080 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_7).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2080 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_8: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            4 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2081 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_8).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2081 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_8).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2081 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_9: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            5 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2082 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_9).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2082 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_9).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2082 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_10: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            6 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2083 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_10).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2083 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_10).arg == 8 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2083 as ::core::ffi::c_int,
                b"check failed: e->arg == (8)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_11: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            7 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2084 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_11).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2084 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_11).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2084 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_12: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            8 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2085 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_12).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2085 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_12).arg == 6 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2085 as ::core::ffi::c_int,
                b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_13: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            9 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2086 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_13).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2086 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_13).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2086 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_14: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            10 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2087 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_14).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2087 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_14).arg == 7 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2087 as ::core::ffi::c_int,
                b"check failed: e->arg == (7)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_15: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            11 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2088 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_15).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2088 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_15).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2088 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_16: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            12 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2089 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_16).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2089 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_16).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2089 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_17: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            13 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2090 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_17).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2090 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_17).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2090 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_18: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            14 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2091 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_18).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2091 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_18).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2091 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_19: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            15 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2092 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_19).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2092 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_19).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2092 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_20: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            16 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2093 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_20).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2093 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_20).arg == 5 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2093 as ::core::ffi::c_int,
                b"check failed: e->arg == (5)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_21: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            17 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2094 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_21).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2094 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_21).arg == 8 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2094 as ::core::ffi::c_int,
                b"check failed: e->arg == (8)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_22: *const handler_record_entry = _handler_record_get(
            &raw mut storage_1,
            18 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2095 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_22).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2095 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_22).arg == 6 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2095 as ::core::ffi::c_int,
                b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !(storage_1.count == 19 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2096 as ::core::ffi::c_int,
                b"check failed: storage.count == 19\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        set_subtest(b"with skip handler\0".as_ptr() as *const ::core::ffi::c_char);
        let mut storage_2: handler_record_list = handler_record_list {
            count: 0,
            entries: [handler_record_entry {
                name: ::core::ptr::null::<::core::ffi::c_char>(),
                arg: 0,
            }; 50],
        };
        storage_2.count = 0 as ::core::ffi::c_int;
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        XML_SetDefaultHandler(
            g_parser,
            Some(
                record_default_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                record_cdata_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetSkippedEntityHandler(
            g_parser,
            Some(
                record_skip_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage_2 as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            entity_text,
            strlen(entity_text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2112 as ::core::ffi::c_int,
            );
        }
        let mut e_23: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            0 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2114 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_23).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2114 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_23).arg == 9 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2114 as ::core::ffi::c_int,
                b"check failed: e->arg == (9)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_24: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            1 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2115 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_24).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2115 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_24).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2115 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_25: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            2 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2116 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_25).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2116 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_25).arg == 3 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2116 as ::core::ffi::c_int,
                b"check failed: e->arg == (3)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_26: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            3 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2117 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_26).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2117 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_26).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2117 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_27: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            4 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2118 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_27).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2118 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_27).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2118 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_28: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            5 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2119 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_28).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2119 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_28).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2119 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_29: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            6 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2120 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_29).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2120 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_29).arg == 8 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2120 as ::core::ffi::c_int,
                b"check failed: e->arg == (8)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_30: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            7 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2121 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_30).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2121 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_30).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2121 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_31: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            8 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2122 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_31).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2122 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_31).arg == 6 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2122 as ::core::ffi::c_int,
                b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_32: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            9 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2123 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_32).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2123 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_32).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2123 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_33: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            10 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2124 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_33).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2124 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_33).arg == 7 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2124 as ::core::ffi::c_int,
                b"check failed: e->arg == (7)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_34: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            11 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2125 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_34).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2125 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_34).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2125 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_35: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            12 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2126 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_35).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2126 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_35).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2126 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_36: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            13 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2127 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_36).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2127 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_36).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2127 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_37: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            14 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2128 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_37).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2128 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_37).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2128 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_38: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            15 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2129 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_38).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2129 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_38).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2129 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_39: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            16 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2130 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_39).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2130 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_39).arg == 5 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2130 as ::core::ffi::c_int,
                b"check failed: e->arg == (5)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_40: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            17 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2131 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_40).name,
            b"record_skip_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2131 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_skip_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_40).arg == 0 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2131 as ::core::ffi::c_int,
                b"check failed: e->arg == (0)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_41: *const handler_record_entry = _handler_record_get(
            &raw mut storage_2,
            18 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2132 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_41).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2132 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_41).arg == 6 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2132 as ::core::ffi::c_int,
                b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !(storage_2.count == 19 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2133 as ::core::ffi::c_int,
                b"check failed: storage.count == 19\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        set_subtest(b"allow entity\0".as_ptr() as *const ::core::ffi::c_char);
        let mut storage_3: handler_record_list = handler_record_list {
            count: 0,
            entries: [handler_record_entry {
                name: ::core::ptr::null::<::core::ffi::c_char>(),
                arg: 0,
            }; 50],
        };
        storage_3.count = 0 as ::core::ffi::c_int;
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        XML_SetDefaultHandlerExpand(
            g_parser,
            Some(
                record_default_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                record_cdata_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage_3 as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            entity_text,
            strlen(entity_text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2148 as ::core::ffi::c_int,
            );
        }
        let mut e_42: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            0 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2149 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_42).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2149 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_42).arg == 9 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2149 as ::core::ffi::c_int,
                b"check failed: e->arg == (9)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_43: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            1 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2150 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_43).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2150 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_43).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2150 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_44: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            2 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2151 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_44).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2151 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_44).arg == 3 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2151 as ::core::ffi::c_int,
                b"check failed: e->arg == (3)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_45: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            3 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2152 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_45).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2152 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_45).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2152 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_46: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            4 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2153 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_46).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2153 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_46).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2153 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_47: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            5 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2154 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_47).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2154 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_47).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2154 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_48: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            6 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2155 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_48).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2155 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_48).arg == 8 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2155 as ::core::ffi::c_int,
                b"check failed: e->arg == (8)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_49: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            7 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2156 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_49).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2156 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_49).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2156 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_50: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            8 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2157 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_50).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2157 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_50).arg == 6 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2157 as ::core::ffi::c_int,
                b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_51: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            9 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2158 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_51).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2158 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_51).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2158 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_52: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            10 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2159 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_52).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2159 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_52).arg == 7 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2159 as ::core::ffi::c_int,
                b"check failed: e->arg == (7)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_53: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            11 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2160 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_53).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2160 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_53).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2160 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_54: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            12 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2161 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_54).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2161 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_54).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2161 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_55: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            13 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2162 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_55).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2162 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_55).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2162 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_56: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            14 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2163 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_56).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2163 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_56).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2163 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_57: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            15 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2164 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_57).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2164 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_57).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2164 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_58: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            16 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2165 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_58).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2165 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_58).arg == 5 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2165 as ::core::ffi::c_int,
                b"check failed: e->arg == (5)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_59: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            17 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2166 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_59).name,
            b"record_cdata_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2166 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_cdata_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_59).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2166 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_60: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            18 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2167 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_60).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2167 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_60).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2167 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_61: *const handler_record_entry = _handler_record_get(
            &raw mut storage_3,
            19 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2168 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_61).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2168 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_61).arg == 6 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2168 as ::core::ffi::c_int,
                b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !(storage_3.count == 20 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2169 as ::core::ffi::c_int,
                b"check failed: storage.count == 20\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        set_subtest(b"not passing cdata\0".as_ptr() as *const ::core::ffi::c_char);
        let mut storage_4: handler_record_list = handler_record_list {
            count: 0,
            entries: [handler_record_entry {
                name: ::core::ptr::null::<::core::ffi::c_char>(),
                arg: 0,
            }; 50],
        };
        storage_4.count = 0 as ::core::ffi::c_int;
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        XML_SetDefaultHandlerExpand(
            g_parser,
            Some(
                record_default_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                record_cdata_nodefault_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage_4 as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            entity_text,
            strlen(entity_text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2184 as ::core::ffi::c_int,
            );
        }
        let mut e_62: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            0 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2185 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_62).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2185 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_62).arg == 9 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2185 as ::core::ffi::c_int,
                b"check failed: e->arg == (9)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_63: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            1 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2186 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_63).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2186 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_63).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2186 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_64: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            2 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2187 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_64).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2187 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_64).arg == 3 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2187 as ::core::ffi::c_int,
                b"check failed: e->arg == (3)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_65: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            3 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2188 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_65).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2188 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_65).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2188 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_66: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            4 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2189 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_66).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2189 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_66).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2189 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_67: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            5 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2190 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_67).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2190 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_67).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2190 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_68: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            6 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2191 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_68).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2191 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_68).arg == 8 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2191 as ::core::ffi::c_int,
                b"check failed: e->arg == (8)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_69: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            7 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2192 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_69).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2192 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_69).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2192 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_70: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            8 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2193 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_70).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2193 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_70).arg == 6 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2193 as ::core::ffi::c_int,
                b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_71: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            9 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2194 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_71).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2194 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_71).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2194 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_72: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            10 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2195 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_72).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2195 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_72).arg == 7 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2195 as ::core::ffi::c_int,
                b"check failed: e->arg == (7)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_73: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            11 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2196 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_73).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2196 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_73).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2196 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_74: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            12 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2197 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_74).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2197 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_74).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2197 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_75: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            13 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2198 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_75).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2198 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_75).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2198 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_76: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            14 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2199 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_76).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2199 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_76).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2199 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_77: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            15 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2200 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_77).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2200 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_77).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2200 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_78: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            16 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2201 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_78).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2201 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_78).arg == 5 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2201 as ::core::ffi::c_int,
                b"check failed: e->arg == (5)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_79: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            17 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2203 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_79).name,
            b"record_cdata_nodefault_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2203 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_cdata_nodefault_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_79).arg == 1 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2203 as ::core::ffi::c_int,
                b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut e_80: *const handler_record_entry = _handler_record_get(
            &raw mut storage_4,
            18 as ::core::ffi::c_int,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2204 as ::core::ffi::c_int,
        );
        if !(strcmp(
            (*e_80).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2204 as ::core::ffi::c_int,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_80).arg == 6 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2204 as ::core::ffi::c_int,
                b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !(storage_4.count == 19 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2205 as ::core::ffi::c_int,
                b"check failed: storage.count == 19\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_dtd_elements() {
    set_test_info(b"test_dtd_elements\0", 2211 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(
        b"<!DOCTYPE doc [\n<!ELEMENT doc (chapter)>\n<!ELEMENT chapter (#PCDATA)>\n]>\n<doc><chapter>Wombats are go</chapter></doc>\0",
    );
    ffi_call2(
        XML_SetElementDeclHandler,
        current_parser(),
        Some(
            dummy_element_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Content,
                ) -> (),
        ),
    );
    ensure_parser_success(
        parse_single_bytes_c_string(text),
        2221 as ::core::ffi::c_int,
    );
}
extern "C" fn element_decl_check_model(
    _user_data: *mut ::core::ffi::c_void,
    name: *const XML_Char,
    model: *mut XML_Content,
) {
    let mut error_flags = 0 as uint32_t;
    error_flags |= if xml_name_eq!(name, b"junk\0") {
        0
    } else {
        (1 as uint32_t) << 0
    };
    error_flags |= if !model.is_null() {
        0
    } else {
        (1 as uint32_t) << 1
    };
    if !model.is_null() {
        let nodes = slice_from_raw_parts(model, 6);
        let root = &nodes[0];
        let choice = &nodes[1];
        let zebra = &nodes[2];
        let bar = &nodes[3];
        let foo = &nodes[4];
        let xyz = &nodes[5];

        error_flags |= if root.type_0 == XML_CTYPE_SEQ {
            0
        } else {
            (1 as uint32_t) << 2
        };
        error_flags |= if root.quant == XML_CQUANT_NONE {
            0
        } else {
            (1 as uint32_t) << 3
        };
        error_flags |= if root.numchildren == 2 as ::core::ffi::c_uint {
            0
        } else {
            (1 as uint32_t) << 4
        };
        error_flags |= if root.children == nodes.as_ptr().wrapping_add(1) as *mut XML_Content {
            0
        } else {
            (1 as uint32_t) << 5
        };
        error_flags |= if root.name.is_null() {
            0
        } else {
            (1 as uint32_t) << 6
        };

        error_flags |= if choice.type_0 == XML_CTYPE_CHOICE {
            0
        } else {
            (1 as uint32_t) << 7
        };
        error_flags |= if choice.quant == XML_CQUANT_NONE {
            0
        } else {
            (1 as uint32_t) << 8
        };
        error_flags |= if choice.numchildren == 3 as ::core::ffi::c_uint {
            0
        } else {
            (1 as uint32_t) << 9
        };
        error_flags |= if choice.children == nodes.as_ptr().wrapping_add(3) as *mut XML_Content {
            0
        } else {
            (1 as uint32_t) << 10
        };
        error_flags |= if choice.name.is_null() {
            0
        } else {
            (1 as uint32_t) << 11
        };

        error_flags |= if zebra.type_0 == XML_CTYPE_NAME {
            0
        } else {
            (1 as uint32_t) << 12
        };
        error_flags |= if zebra.quant == XML_CQUANT_REP {
            0
        } else {
            (1 as uint32_t) << 13
        };
        error_flags |= if zebra.numchildren == 0 as ::core::ffi::c_uint {
            0
        } else {
            (1 as uint32_t) << 14
        };
        error_flags |= if zebra.children.is_null() {
            0
        } else {
            (1 as uint32_t) << 15
        };
        error_flags |= if xml_name_eq!(zebra.name, b"zebra\0") {
            0
        } else {
            (1 as uint32_t) << 16
        };

        error_flags |= if bar.type_0 == XML_CTYPE_NAME {
            0
        } else {
            (1 as uint32_t) << 17
        };
        error_flags |= if bar.quant == XML_CQUANT_NONE {
            0
        } else {
            (1 as uint32_t) << 18
        };
        error_flags |= if bar.numchildren == 0 as ::core::ffi::c_uint {
            0
        } else {
            (1 as uint32_t) << 19
        };
        error_flags |= if bar.children.is_null() {
            0
        } else {
            (1 as uint32_t) << 20
        };
        error_flags |= if xml_name_eq!(bar.name, b"bar\0") {
            0
        } else {
            (1 as uint32_t) << 21
        };

        error_flags |= if foo.type_0 == XML_CTYPE_NAME {
            0
        } else {
            (1 as uint32_t) << 22
        };
        error_flags |= if foo.quant == XML_CQUANT_NONE {
            0
        } else {
            (1 as uint32_t) << 23
        };
        error_flags |= if foo.numchildren == 0 as ::core::ffi::c_uint {
            0
        } else {
            (1 as uint32_t) << 24
        };
        error_flags |= if foo.children.is_null() {
            0
        } else {
            (1 as uint32_t) << 25
        };
        error_flags |= if xml_name_eq!(foo.name, b"foo\0") {
            0
        } else {
            (1 as uint32_t) << 26
        };

        error_flags |= if xyz.type_0 == XML_CTYPE_NAME {
            0
        } else {
            (1 as uint32_t) << 27
        };
        error_flags |= if xyz.quant == XML_CQUANT_PLUS {
            0
        } else {
            (1 as uint32_t) << 28
        };
        error_flags |= if xyz.numchildren == 0 as ::core::ffi::c_uint {
            0
        } else {
            (1 as uint32_t) << 29
        };
        error_flags |= if xyz.children.is_null() {
            0
        } else {
            (1 as uint32_t) << 30
        };
        error_flags |= if xml_name_eq!(xyz.name, b"xyz\0") {
            0
        } else {
            (1 as uint32_t) << 31
        };
    }
    ffi_call2(
        XML_SetUserData,
        current_parser(),
        error_flags as uintptr_t as *mut ::core::ffi::c_void,
    );
    ffi_call2(XML_FreeContentModel, current_parser(), model);
}
extern "C" fn test_dtd_elements_nesting() {
    set_test_info(b"test_dtd_elements_nesting\0", 2285 as ::core::ffi::c_int);
    let parser = current_parser();
    let text = bytes_as_c_char_ptr(
        b"<!DOCTYPE foo [\n<!ELEMENT junk ((bar|foo|xyz+), zebra*)>\n]>\n<foo/>\0",
    );
    ffi_call2(
        XML_SetUserData,
        parser,
        -(1 as ::core::ffi::c_int) as uintptr_t as *mut ::core::ffi::c_void,
    );
    ffi_call2(
        XML_SetElementDeclHandler,
        parser,
        Some(
            element_decl_check_model
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Content,
                ) -> (),
        ),
    );
    ensure_parser_success(
        parse_single_bytes_c_string(text),
        2297 as ::core::ffi::c_int,
    );
    if parser_user_data_bits(parser) != 0 as uint32_t {
        fail_test(
            2300 as ::core::ffi::c_int,
            b"Element declaration model regression detected\0",
        );
    }
}
extern "C" fn test_set_foreign_dtd() {
    set_test_info(b"test_set_foreign_dtd\0", 2305 as ::core::ffi::c_int);
    let text1 = bytes_as_c_char_ptr(b"<?xml version='1.0' encoding='us-ascii'?>\n\0");
    let text2 = bytes_as_c_char_ptr(b"<doc>&entity;</doc>\0");
    let mut test_data = ExtTest {
        parse_text: bytes_as_c_char_ptr(b"<!ELEMENT doc (#PCDATA)*>\0"),
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };

    parser_set_hash_salt(0x12345678 as ::core::ffi::c_ulong);
    parser_set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
    parser_set_user_data((&raw mut test_data).cast());
    parser_set_external_entity_ref_handler(external_entity_loader_handler_for_tests());
    parser_set_default_handler(default_handler());

    assert_test_condition(
        parser_use_foreign_dtd(XML_TRUE) as ::core::ffi::c_uint
            == XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint,
        2318 as ::core::ffi::c_int,
        b"Could not set foreign DTD\0",
    );

    ensure_parser_success(
        parse_single_bytes_with_final(text1, c_string_len(text1), XML_FALSE as ::core::ffi::c_int),
        2321 as ::core::ffi::c_int,
    );

    assert_test_condition(
        parser_use_foreign_dtd(XML_TRUE) as ::core::ffi::c_uint
            == XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        2328 as ::core::ffi::c_int,
        b"Failed to reject late foreign DTD setting\0",
    );
    assert_test_condition(
        parser_set_hash_salt(0x23456789 as ::core::ffi::c_ulong) == 0,
        2331 as ::core::ffi::c_int,
        b"Failed to reject late hash salt change\0",
    );

    ensure_parser_success(
        parse_single_bytes_with_final(text2, c_string_len(text2), XML_TRUE as ::core::ffi::c_int),
        2336 as ::core::ffi::c_int,
    );
}
extern "C" fn test_foreign_dtd_not_standalone() {
    set_test_info(
        b"test_foreign_dtd_not_standalone\0",
        2341 as ::core::ffi::c_int,
    );
    let text =
        bytes_as_c_char_ptr(b"<?xml version='1.0' encoding='us-ascii'?>\n<doc>&entity;</doc>\0");
    let mut test_data = ExtTest {
        parse_text: bytes_as_c_char_ptr(b"<!ELEMENT doc (#PCDATA)*>\0"),
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };

    parser_set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
    parser_set_user_data((&raw mut test_data).cast());
    parser_set_external_entity_ref_handler(external_entity_loader_handler_for_tests());
    parser_set_not_standalone_handler(reject_not_standalone_handler_for_tests());

    assert_test_condition(
        parser_use_foreign_dtd(XML_TRUE) as ::core::ffi::c_uint
            == XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint,
        2351 as ::core::ffi::c_int,
        b"Could not set foreign DTD\0",
    );
    expect_failure(
        text,
        XML_ERROR_NOT_STANDALONE,
        b"NotStandalonehandler failed to reject\0",
        2353 as ::core::ffi::c_int,
    );
}
extern "C" fn test_invalid_foreign_dtd() {
    set_test_info(b"test_invalid_foreign_dtd\0", 2358 as ::core::ffi::c_int);
    let text =
        bytes_as_c_char_ptr(b"<?xml version='1.0' encoding='us-ascii'?>\n<doc>&entity;</doc>\0");
    let mut test_data = ext_faults {
        parse_text: bytes_as_c_char_ptr(b"$\0"),
        fail_text: bytes_as_c_char_ptr(b"Dollar not faulted\0"),
        encoding: ::core::ptr::null::<XML_Char>(),
        error: XML_ERROR_INVALID_TOKEN,
    };

    parser_set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
    parser_set_user_data((&raw mut test_data).cast());
    parser_set_external_entity_ref_handler(external_entity_faulter_handler_for_tests());
    parser_use_foreign_dtd(XML_TRUE);
    expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Bad DTD should not have been accepted\0",
        2369 as ::core::ffi::c_int,
    );
}
extern "C" fn test_foreign_dtd_with_doctype() {
    set_test_info(
        b"test_foreign_dtd_with_doctype\0",
        2374 as ::core::ffi::c_int,
    );
    let text1 = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc [<!ENTITY entity 'hello world'>]>\n\0",
    );
    let text2 = bytes_as_c_char_ptr(b"<doc>&entity;</doc>\0");
    let mut test_data = ExtTest {
        parse_text: bytes_as_c_char_ptr(b"<!ELEMENT doc (#PCDATA)*>\0"),
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };

    parser_set_hash_salt(0x12345678 as ::core::ffi::c_ulong);
    parser_set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
    parser_set_user_data((&raw mut test_data).cast());
    parser_set_external_entity_ref_handler(external_entity_loader_handler_for_tests());
    parser_set_default_handler(default_handler());

    assert_test_condition(
        parser_use_foreign_dtd(XML_TRUE) as ::core::ffi::c_uint
            == XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint,
        2388 as ::core::ffi::c_int,
        b"Could not set foreign DTD\0",
    );

    ensure_parser_success(
        parse_single_bytes_with_final(text1, c_string_len(text1), XML_FALSE as ::core::ffi::c_int),
        2391 as ::core::ffi::c_int,
    );

    assert_test_condition(
        parser_use_foreign_dtd(XML_TRUE) as ::core::ffi::c_uint
            == XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        2398 as ::core::ffi::c_int,
        b"Failed to reject late foreign DTD setting\0",
    );
    assert_test_condition(
        parser_set_hash_salt(0x23456789 as ::core::ffi::c_ulong) == 0,
        2401 as ::core::ffi::c_int,
        b"Failed to reject late hash salt change\0",
    );

    ensure_parser_success(
        parse_single_bytes_with_final(text2, c_string_len(text2), XML_TRUE as ::core::ffi::c_int),
        2406 as ::core::ffi::c_int,
    );
}
extern "C" fn test_foreign_dtd_without_external_subset() {
    set_test_info(
        b"test_foreign_dtd_without_external_subset\0",
        2411 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(b"<!DOCTYPE doc [<!ENTITY foo 'bar'>]>\n<doc>&foo;</doc>\0");

    parser_set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
    parser_set_user_data(NULL);
    parser_set_external_entity_ref_handler(external_entity_null_loader_handler_for_tests());
    parser_use_foreign_dtd(XML_TRUE);
    ensure_parser_success(
        parse_single_bytes_with_final(text, c_string_len(text), XML_TRUE as ::core::ffi::c_int),
        2421 as ::core::ffi::c_int,
    );
}
extern "C" fn test_empty_foreign_dtd() {
    set_test_info(b"test_empty_foreign_dtd\0", 2425 as ::core::ffi::c_int);
    let text =
        bytes_as_c_char_ptr(b"<?xml version='1.0' encoding='us-ascii'?>\n<doc>&entity;</doc>\0");

    parser_set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
    parser_set_external_entity_ref_handler(external_entity_null_loader_handler_for_tests());
    parser_use_foreign_dtd(XML_TRUE);
    expect_failure(
        text,
        XML_ERROR_UNDEFINED_ENTITY,
        b"Undefined entity not faulted\0",
        2433 as ::core::ffi::c_int,
    );
}
extern "C" fn test_set_base() {
    set_test_info(b"test_set_base\0", 2438 as ::core::ffi::c_int);
    let old_base = parser_base();
    let new_base = bytes_as_xml_char_ptr(b"/local/file/name.xml\0");

    assert_test_condition(
        parser_set_base(new_base) as ::core::ffi::c_uint
            == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint,
        2444 as ::core::ffi::c_int,
        b"Unable to set base\0",
    );
    assert_test_condition(
        ffi_call2(strcmp, parser_base().cast(), new_base.cast()) == 0 as ::core::ffi::c_int,
        2446 as ::core::ffi::c_int,
        b"Base setting not correct\0",
    );
    assert_test_condition(
        parser_set_base(::core::ptr::null::<XML_Char>()) as ::core::ffi::c_uint
            == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint,
        2448 as ::core::ffi::c_int,
        b"Unable to NULL base\0",
    );
    assert_test_condition(
        parser_base().is_null(),
        2450 as ::core::ffi::c_int,
        b"Base setting not nulled\0",
    );

    parser_set_base(old_base);
}
extern "C" fn test_attributes() {
    unsafe {
        _check_set_test_info(
            b"test_attributes\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2456 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc (tag)>\n<!ATTLIST doc id ID #REQUIRED>\n]><doc a='1' id='one' b='2'><tag c='3'/></doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut doc_info: [AttrInfo; 4] = [
            attrInfo {
                name: b"a\0".as_ptr() as *const XML_Char,
                value: b"1\0".as_ptr() as *const XML_Char,
            },
            attrInfo {
                name: b"b\0".as_ptr() as *const XML_Char,
                value: b"2\0".as_ptr() as *const XML_Char,
            },
            attrInfo {
                name: b"id\0".as_ptr() as *const XML_Char,
                value: b"one\0".as_ptr() as *const XML_Char,
            },
            attrInfo {
                name: ::core::ptr::null::<XML_Char>(),
                value: ::core::ptr::null::<XML_Char>(),
            },
        ];
        let mut tag_info: [AttrInfo; 2] = [
            attrInfo {
                name: b"c\0".as_ptr() as *const XML_Char,
                value: b"3\0".as_ptr() as *const XML_Char,
            },
            attrInfo {
                name: ::core::ptr::null::<XML_Char>(),
                value: ::core::ptr::null::<XML_Char>(),
            },
        ];
        let mut info: [ElementInfo; 3] = [
            elementInfo {
                name: b"doc\0".as_ptr() as *const XML_Char,
                attr_count: 3 as ::core::ffi::c_int,
                id_name: b"id\0".as_ptr() as *const XML_Char,
                attributes: ::core::ptr::null_mut::<AttrInfo>(),
            },
            elementInfo {
                name: b"tag\0".as_ptr() as *const XML_Char,
                attr_count: 1 as ::core::ffi::c_int,
                id_name: ::core::ptr::null::<XML_Char>(),
                attributes: ::core::ptr::null_mut::<AttrInfo>(),
            },
            elementInfo {
                name: ::core::ptr::null::<XML_Char>(),
                attr_count: 0 as ::core::ffi::c_int,
                id_name: ::core::ptr::null::<XML_Char>(),
                attributes: ::core::ptr::null_mut::<AttrInfo>(),
            },
        ];
        info[0 as ::core::ffi::c_int as usize].attributes = &raw mut doc_info as *mut AttrInfo;
        info[1 as ::core::ffi::c_int as usize].attributes = &raw mut tag_info as *mut AttrInfo;
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if parser.is_null() {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2476 as ::core::ffi::c_int,
                b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut parserAndElementInfos: ParserAndElementInfo = StructParserAndElementInfo {
            parser: parser,
            info: &raw mut info as *mut ElementInfo,
        };
        XML_SetStartElementHandler(
            parser,
            Some(
                counting_start_element_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetUserData(
            parser,
            &raw mut parserAndElementInfos as *mut ::core::ffi::c_void,
        );
        if _XML_Parse_SINGLE_BYTES(
            parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2486 as ::core::ffi::c_int,
            );
        }
        XML_ParserFree(parser);
    }
}
extern "C" fn test_reset_in_entity() {
    set_test_info(b"test_reset_in_entity\0", 2495 as ::core::ffi::c_int);
    if current_chunk_size() != 0 as ::core::ffi::c_int {
        return;
    }

    let text = bytes_as_c_char_ptr(
        b"<!DOCTYPE doc [\n<!ENTITY wombat 'wom'>\n<!ENTITY entity 'hi &wom; there'>\n]>\n<doc>&entity;</doc>\0",
    );
    configure_resumable_character_data_handler(XML_TRUE);
    ensure_parser_success(parser_parse_c_string(text), 2514 as ::core::ffi::c_int);

    if parser_parsing_status().parsing != XML_SUSPENDED {
        fail_test(
            2517 as ::core::ffi::c_int,
            b"Parsing status not SUSPENDED\0",
        );
    }

    parser_reset();

    if parser_parsing_status().parsing != XML_INITIALIZED {
        fail_test(
            2521 as ::core::ffi::c_int,
            b"Parsing status doesn't reset to INITIALIZED\0",
        );
    }
}
extern "C" fn test_resume_invalid_parse() {
    set_test_info(b"test_resume_invalid_parse\0", 2526 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(b"<doc>Hello</doc\0");

    configure_resumable_character_data_handler(XML_TRUE);
    ensure_parser_success(parser_parse_c_string(text), 2533 as ::core::ffi::c_int);

    if parser_resume() as ::core::ffi::c_uint
        == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            2535 as ::core::ffi::c_int,
            b"Resumed invalid parse not faulted\0",
        );
    }

    if parser_error_code() as ::core::ffi::c_uint
        != XML_ERROR_UNCLOSED_TOKEN as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            2537 as ::core::ffi::c_int,
            b"Invalid parse not correctly faulted\0",
        );
    }
}
extern "C" fn test_resume_resuspended() {
    set_test_info(b"test_resume_resuspended\0", 2542 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(b"<doc>Hello<meep/>world</doc>\0");

    configure_resumable_character_data_handler(XML_TRUE);
    ensure_parser_success(parser_parse_c_string(text), 2549 as ::core::ffi::c_int);

    configure_resumable_character_data_handler(XML_TRUE);
    if parser_resume() as ::core::ffi::c_uint
        != XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(2553 as ::core::ffi::c_int, b"Resumption not suspended\0");
    }

    if parser_resume() as ::core::ffi::c_uint
        != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(2556 as ::core::ffi::c_int);
    }
}
extern "C" fn test_cdata_default() {
    set_test_info(b"test_cdata_default\0", 2561 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(b"<doc><![CDATA[Hello\nworld]]></doc>\0");
    let expected = bytes_as_xml_char_ptr(b"<doc><![CDATA[Hello\nworld]]></doc>\0");
    let mut storage = CharData {
        count: 0,
        data: [0; 2048],
    };

    char_data_init(&mut storage);
    parser_set_user_data((&mut storage as *mut CharData).cast());
    parser_set_default_handler(accumulating_default_handler());
    ensure_parser_success(
        parse_single_bytes_c_string(text),
        2572 as ::core::ffi::c_int,
    );
    char_data_check_xml_chars(&mut storage, expected);
}
extern "C" fn test_subordinate_reset() {
    set_test_info(b"test_subordinate_reset\0", 2578 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0",
    );

    configure_subordinate_external_entity_parser(
        external_entity_resetter_handler_for_tests(),
        None,
    );
    ensure_parser_success(
        parse_single_bytes_c_string(text),
        2587 as ::core::ffi::c_int,
    );
}
extern "C" fn test_subordinate_suspend() {
    set_test_info(b"test_subordinate_suspend\0", 2592 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0",
    );

    configure_subordinate_external_entity_parser(
        external_entity_suspender_handler_for_tests(),
        None,
    );
    ensure_parser_success(
        parse_single_bytes_c_string(text),
        2601 as ::core::ffi::c_int,
    );
}
extern "C" fn test_subordinate_xdecl_suspend() {
    set_test_info(
        b"test_subordinate_xdecl_suspend\0",
        2608 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<!DOCTYPE doc [\n  <!ENTITY entity SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&entity;</doc>\0",
    );

    configure_subordinate_external_entity_parser(
        external_entity_suspend_xmldecl_handler_for_tests(),
        Some(XML_TRUE),
    );
    ensure_parser_success(
        parse_single_bytes_c_string(text),
        2620 as ::core::ffi::c_int,
    );
}
extern "C" fn test_subordinate_xdecl_abort() {
    set_test_info(
        b"test_subordinate_xdecl_abort\0",
        2624 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<!DOCTYPE doc [\n  <!ENTITY entity SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&entity;</doc>\0",
    );

    configure_subordinate_external_entity_parser(
        external_entity_suspend_xmldecl_handler_for_tests(),
        Some(XML_FALSE),
    );
    ensure_parser_success(
        parse_single_bytes_c_string(text),
        2636 as ::core::ffi::c_int,
    );
}
extern "C" fn test_ext_entity_invalid_suspended_parse() {
    set_test_info(
        b"test_ext_entity_invalid_suspended_parse\0",
        2641 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0",
    );
    let faults = [
        ext_faults {
            parse_text: bytes_as_c_char_ptr(b"<?xml version='1.0' encoding='us-ascii'?><\0"),
            fail_text: bytes_as_c_char_ptr(b"Incomplete element declaration not faulted\0"),
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_UNCLOSED_TOKEN,
        },
        ext_faults {
            parse_text: bytes_as_c_char_ptr(b"<?xml version='1.0' encoding='utf-8'?>\xE2\x82\0"),
            fail_text: bytes_as_c_char_ptr(b"Incomplete character not faulted\0"),
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_PARTIAL_CHAR,
        },
        ext_faults {
            parse_text: ::core::ptr::null::<::core::ffi::c_char>(),
            fail_text: ::core::ptr::null::<::core::ffi::c_char>(),
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_NONE,
        },
    ];

    for fault in faults
        .iter()
        .take_while(|fault| !fault.parse_text.is_null())
    {
        set_subtest_text(fault.parse_text);
        configure_subordinate_external_entity_parser(
            external_entity_suspending_faulter_handler_for_tests(),
            None,
        );
        parser_set_user_data((fault as *const ExtFaults).cast_mut().cast());
        expect_failure(
            text,
            XML_ERROR_EXTERNAL_ENTITY_HANDLING,
            b"Parser did not report external entity error\0",
            2663 as ::core::ffi::c_int,
        );
        parser_reset();
    }
}
extern "C" fn test_explicit_encoding() {
    unsafe {
        _check_set_test_info(
            b"test_explicit_encoding\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2670 as ::core::ffi::c_int,
        );
        let mut text1: *const ::core::ffi::c_char =
            b"<doc>Hello \0".as_ptr() as *const ::core::ffi::c_char;
        let mut text2: *const ::core::ffi::c_char =
            b" World</doc>\0".as_ptr() as *const ::core::ffi::c_char;
        if XML_SetEncoding(g_parser, ::core::ptr::null::<XML_Char>()) as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2676 as ::core::ffi::c_int,
                b"Failed to initialise encoding to NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_SetEncoding(g_parser, b"utf-8\0".as_ptr() as *const XML_Char) as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2679 as ::core::ffi::c_int,
                b"Failed to set explicit encoding\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text1,
            strlen(text1) as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2682 as ::core::ffi::c_int,
            );
        }
        if XML_SetEncoding(g_parser, b"us-ascii\0".as_ptr() as *const XML_Char)
            as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2685 as ::core::ffi::c_int,
                b"Allowed encoding change\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text2,
            strlen(text2) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2688 as ::core::ffi::c_int,
            );
        }
        if XML_SetEncoding(g_parser, ::core::ptr::null::<XML_Char>()) as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2691 as ::core::ffi::c_int,
                b"Failed to unset encoding\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_trailing_cr() {
    unsafe {
        _check_set_test_info(
            b"test_trailing_cr\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2696 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc>\r\0".as_ptr() as *const ::core::ffi::c_char;
        let mut found_cr: ::core::ffi::c_int = 0;
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                cr_cdata_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut found_cr as *mut ::core::ffi::c_void);
        found_cr = 0 as ::core::ffi::c_int;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2706 as ::core::ffi::c_int,
                b"Failed to fault unclosed doc\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if found_cr == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2708 as ::core::ffi::c_int,
                b"Did not catch the carriage return\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        XML_SetDefaultHandler(
            g_parser,
            Some(
                cr_cdata_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut found_cr as *mut ::core::ffi::c_void);
        found_cr = 0 as ::core::ffi::c_int;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2717 as ::core::ffi::c_int,
                b"Failed to fault unclosed doc\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if found_cr == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2719 as ::core::ffi::c_int,
                b"Did not catch default carriage return\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_ext_entity_trailing_cr() {
    unsafe {
        _check_set_test_info(
            b"test_ext_entity_trailing_cr\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2724 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut found_cr: ::core::ffi::c_int = 0;
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_cr_catcher
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, &raw mut found_cr as *mut ::core::ffi::c_void);
        found_cr = 0 as ::core::ffi::c_int;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2737 as ::core::ffi::c_int,
            );
        }
        if found_cr == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2739 as ::core::ffi::c_int,
                b"No carriage return found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_bad_cr_catcher
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, &raw mut found_cr as *mut ::core::ffi::c_void);
        found_cr = 0 as ::core::ffi::c_int;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2749 as ::core::ffi::c_int,
            );
        }
        if found_cr == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2751 as ::core::ffi::c_int,
                b"No carriage return found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_trailing_rsqb() {
    unsafe {
        _check_set_test_info(
            b"test_trailing_rsqb\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2756 as ::core::ffi::c_int,
        );
        let mut text8: *const ::core::ffi::c_char =
            b"<doc>]\0".as_ptr() as *const ::core::ffi::c_char;
        let text16: [::core::ffi::c_char; 15] = ::core::mem::transmute::<
            [u8; 15],
            [::core::ffi::c_char; 15],
        >(*b"\xFF\xFE<\0d\0o\0c\0>\0]\0\0");
        let mut found_rsqb: ::core::ffi::c_int = 0;
        let mut text8_len: ::core::ffi::c_int = strlen(text8) as ::core::ffi::c_int;
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                rsqb_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut found_rsqb as *mut ::core::ffi::c_void);
        found_rsqb = 0 as ::core::ffi::c_int;
        if _XML_Parse_SINGLE_BYTES(g_parser, text8, text8_len, XML_TRUE as ::core::ffi::c_int)
            as ::core::ffi::c_uint
            == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2767 as ::core::ffi::c_int,
                b"Failed to fault unclosed doc\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if found_rsqb == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2769 as ::core::ffi::c_int,
                b"Did not catch the right square bracket\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                rsqb_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut found_rsqb as *mut ::core::ffi::c_void);
        found_rsqb = 0 as ::core::ffi::c_int;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text16 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 15]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2779 as ::core::ffi::c_int,
                b"Failed to fault unclosed doc\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if found_rsqb == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2781 as ::core::ffi::c_int,
                b"Did not catch the right square bracket\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        XML_SetDefaultHandler(
            g_parser,
            Some(
                rsqb_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut found_rsqb as *mut ::core::ffi::c_void);
        found_rsqb = 0 as ::core::ffi::c_int;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text16 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 15]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2791 as ::core::ffi::c_int,
                b"Failed to fault unclosed doc\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if found_rsqb == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2793 as ::core::ffi::c_int,
                b"Did not catch the right square bracket\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_ext_entity_trailing_rsqb() {
    unsafe {
        _check_set_test_info(
            b"test_ext_entity_trailing_rsqb\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2798 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut found_rsqb: ::core::ffi::c_int = 0;
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_rsqb_catcher
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, &raw mut found_rsqb as *mut ::core::ffi::c_void);
        found_rsqb = 0 as ::core::ffi::c_int;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2811 as ::core::ffi::c_int,
            );
        }
        if found_rsqb == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2813 as ::core::ffi::c_int,
                b"No right square bracket found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_ext_entity_good_cdata() {
    set_test_info(b"test_ext_entity_good_cdata\0", 2818 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0",
    );
    parser_set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
    parser_set_external_entity_ref_handler(external_entity_good_cdata_handler_for_tests());
    ensure_parser_success(parser_parse_c_string(text), 2828 as ::core::ffi::c_int);
}
extern "C" fn test_user_parameters() {
    unsafe {
        _check_set_test_info(
            b"test_user_parameters\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2833 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii'?>\n<!-- Primary parse -->\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut epilog: *const ::core::ffi::c_char =
            b"<!-- Back to primary parser -->\n</doc>\0".as_ptr() as *const ::core::ffi::c_char;
        g_comment_count = 0 as ::core::ffi::c_int;
        g_skip_count = 0 as ::core::ffi::c_int;
        g_xdecl_count = 0 as ::core::ffi::c_int;
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetXmlDeclHandler(
            g_parser,
            Some(
                xml_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_param_checker
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetCommentHandler(
            g_parser,
            Some(
                data_check_comment_handler
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
            ),
        );
        XML_SetSkippedEntityHandler(
            g_parser,
            Some(
                param_check_skip_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_UseParserAsHandlerArg(g_parser);
        XML_SetUserData(
            g_parser,
            1 as ::core::ffi::c_int as *mut ::core::ffi::c_void,
        );
        g_handler_data = g_parser as *const ::core::ffi::c_void;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2854 as ::core::ffi::c_int,
            );
        }
        if XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_NEVER) != 0 {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2857 as ::core::ffi::c_int,
                b"Changed param entity parsing policy while parsing\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            epilog,
            strlen(epilog) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2860 as ::core::ffi::c_int,
            );
        }
        if g_comment_count != 3 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2862 as ::core::ffi::c_int,
                b"Comment handler not invoked enough times\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if g_skip_count != 1 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2864 as ::core::ffi::c_int,
                b"Skip handler not invoked enough times\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if g_xdecl_count != 1 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2866 as ::core::ffi::c_int,
                b"XML declaration handler not invoked\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_ext_entity_ref_parameter() {
    unsafe {
        _check_set_test_info(
            b"test_ext_entity_ref_parameter\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2879 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_ref_param_checker
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetExternalEntityRefHandlerArg(g_parser, text as *mut ::core::ffi::c_void);
        g_handler_data = text as *const ::core::ffi::c_void;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2893 as ::core::ffi::c_int,
            );
        }
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_ref_param_checker
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetExternalEntityRefHandlerArg(g_parser, NULL);
        g_handler_data = g_parser as *const ::core::ffi::c_void;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2903 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_empty_parse() {
    unsafe {
        _check_set_test_info(
            b"test_empty_parse\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2908 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc></doc>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut partial: *const ::core::ffi::c_char =
            b"<doc>\0".as_ptr() as *const ::core::ffi::c_char;
        if XML_Parse(
            g_parser,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2913 as ::core::ffi::c_int,
                b"Parsing empty string faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_Parse(
            g_parser,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2915 as ::core::ffi::c_int,
                b"Parsing final empty string not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_NO_ELEMENTS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2917 as ::core::ffi::c_int,
                b"Parsing final empty string faulted for wrong reason\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2923 as ::core::ffi::c_int,
            );
        }
        if XML_Parse(
            g_parser,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2925 as ::core::ffi::c_int,
                b"Parsing final empty string faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            partial,
            strlen(partial) as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2932 as ::core::ffi::c_int,
            );
        }
        if XML_Parse(
            g_parser,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2934 as ::core::ffi::c_int,
                b"Parsing final incomplete empty string not faulted\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_negative_len_parse() {
    unsafe {
        _check_set_test_info(
            b"test_negative_len_parse\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2939 as ::core::ffi::c_int,
        );
        let doc: *const ::core::ffi::c_char = b"<root/>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut isFinal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while isFinal < 2 as ::core::ffi::c_int {
            set_subtest(
                b"isFinal=%d\0".as_ptr() as *const ::core::ffi::c_char,
                isFinal,
            );
            let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
            if XML_GetErrorCode(parser) as ::core::ffi::c_uint
                != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2947 as ::core::ffi::c_int,
                    b"There was not supposed to be any initial parse error.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            let status: XML_Status =
                XML_Parse(parser, doc, -(1 as ::core::ffi::c_int), isFinal) as XML_Status;
            if status as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2952 as ::core::ffi::c_int,
                    b"Negative len was expected to fail the parse but did not.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            if XML_GetErrorCode(parser) as ::core::ffi::c_uint
                != XML_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2955 as ::core::ffi::c_int,
                    b"Parse error does not match XML_ERROR_INVALID_ARGUMENT.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            XML_ParserFree(parser);
            isFinal += 1;
        }
    }
}
extern "C" fn test_negative_len_parse_buffer() {
    unsafe {
        _check_set_test_info(
            b"test_negative_len_parse_buffer\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2963 as ::core::ffi::c_int,
        );
        let doc: *const ::core::ffi::c_char = b"<root/>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut isFinal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while isFinal < 2 as ::core::ffi::c_int {
            set_subtest(
                b"isFinal=%d\0".as_ptr() as *const ::core::ffi::c_char,
                isFinal,
            );
            let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
            if XML_GetErrorCode(parser) as ::core::ffi::c_uint
                != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2971 as ::core::ffi::c_int,
                    b"There was not supposed to be any initial parse error.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            let buffer: *mut ::core::ffi::c_void =
                XML_GetBuffer(parser, strlen(doc) as ::core::ffi::c_int)
                    as *mut ::core::ffi::c_void;
            if buffer.is_null() {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2976 as ::core::ffi::c_int,
                    b"XML_GetBuffer failed.\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            memcpy(buffer, doc as *const ::core::ffi::c_void, strlen(doc));
            let status: XML_Status =
                XML_ParseBuffer(parser, -(1 as ::core::ffi::c_int), isFinal) as XML_Status;
            if status as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2983 as ::core::ffi::c_int,
                    b"Negative len was expected to fail the parse but did not.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            if XML_GetErrorCode(parser) as ::core::ffi::c_uint
                != XML_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2986 as ::core::ffi::c_int,
                    b"Parse error does not match XML_ERROR_INVALID_ARGUMENT.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            XML_ParserFree(parser);
            isFinal += 1;
        }
    }
}
fn get_feature(feature_id: XML_FeatureEnum) -> Option<::core::ffi::c_long> {
    for feature in feature_list() {
        if feature.feature as ::core::ffi::c_uint == feature_id as ::core::ffi::c_uint {
            return Some(feature.value);
        }
    }

    None
}
extern "C" fn test_get_buffer_1() {
    set_test_info(b"test_get_buffer_1\0", 3010 as ::core::ffi::c_int);
    let text = buffer_test_text();
    let context_bytes = get_feature(XML_FEATURE_CONTEXT_BYTES).unwrap_or(0);

    if !current_parser_buffer(-(12 as ::core::ffi::c_int)).is_null() {
        fail_test(
            3017 as ::core::ffi::c_int,
            b"Negative length buffer not failed\0",
        );
    }

    let buffer = current_parser_buffer(1536 as ::core::ffi::c_int);
    if buffer.is_null() {
        fail_test(3022 as ::core::ffi::c_int, b"1.5K buffer failed\0");
    }

    copy_c_string_to_buffer(buffer, text);
    ensure_parser_success(
        current_parser_parse_buffer(c_string_len(text), XML_FALSE as ::core::ffi::c_int),
        3027 as ::core::ffi::c_int,
    );

    if !current_parser_buffer(INT_MAX).is_null() {
        fail_test(3029 as ::core::ffi::c_int, b"INT_MAX buffer not failed\0");
    }

    let almost_int_max = (INT_MAX as ::core::ffi::c_long
        - (context_bytes + 1025 as ::core::ffi::c_long))
        as ::core::ffi::c_int;
    if !current_parser_buffer(almost_int_max).is_null() {
        fail_test(3044 as ::core::ffi::c_int, b"INT_MAX- buffer not failed\0");
    }

    if current_parser_buffer(1000 as ::core::ffi::c_int).is_null() {
        fail_test(3048 as ::core::ffi::c_int, b"1000 buffer failed\0");
    }
}
extern "C" fn test_get_buffer_2() {
    set_test_info(b"test_get_buffer_2\0", 3053 as ::core::ffi::c_int);
    let text = buffer_test_text();

    let buffer = current_parser_buffer(1536 as ::core::ffi::c_int);
    if buffer.is_null() {
        fail_test(3060 as ::core::ffi::c_int, b"1.5K buffer failed\0");
    }

    copy_c_string_to_buffer(buffer, text);
    ensure_parser_success(
        current_parser_parse_buffer(c_string_len(text), XML_FALSE as ::core::ffi::c_int),
        3065 as ::core::ffi::c_int,
    );

    if current_parser_buffer(1024 as ::core::ffi::c_int).is_null() {
        fail_test(3069 as ::core::ffi::c_int, b"1024 buffer failed\0");
    }
}
extern "C" fn test_get_buffer_3_overflow() {
    set_test_info(b"test_get_buffer_3_overflow\0", 3075 as ::core::ffi::c_int);
    let parser = create_parser_or_fail(3077 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(b"\n\0");
    let expected_keep_value = c_string_len(text);

    if parser_status_is_error(ffi_call4(
        _XML_Parse_SINGLE_BYTES,
        parser,
        text,
        c_string_len(text),
        XML_FALSE as ::core::ffi::c_int,
    )) {
        xml_failure_for(parser, 3087 as ::core::ffi::c_int);
    }

    assert!(expected_keep_value > 0 as ::core::ffi::c_int);
    if !parser_buffer_for(
        parser,
        INT_MAX - expected_keep_value + 1 as ::core::ffi::c_int,
    )
    .is_null()
    {
        fail_test(3091 as ::core::ffi::c_int, b"enlarging buffer not failed\0");
    }

    parser_free(parser);
}
extern "C" fn test_buffer_can_grow_to_max() {
    set_test_info(b"test_buffer_can_grow_to_max\0", 3098 as ::core::ffi::c_int);
    let prefixes = [
        bytes_as_c_char_ptr(b"\0"),
        bytes_as_c_char_ptr(b"<\0"),
        bytes_as_c_char_ptr(b"<x a='\0"),
        bytes_as_c_char_ptr(b"<doc><x a='\0"),
        bytes_as_c_char_ptr(b"<document><x a='\0"),
        bytes_as_c_char_ptr(b"<averylongelementnamesuchthatitwillhopefullystretchacrossmultiplelinesandlookprettyridiculousitsalsoveryhardtoreadandifyouredoingitihavetowonderifyoureallydonthaveanythingbettertodoofcourseiguessicouldveputsomethingbadinherebutipromisethatididntheybtwhowgreatarespacesandpunctuationforhelpingwithreadabilityprettygreatithinkanywaysthisisprobablylongenoughbye><x a='\0"),
    ];
    let maxbuf = INT_MAX / 2 as ::core::ffi::c_int + (INT_MAX & 1 as ::core::ffi::c_int);

    for prefix in prefixes {
        set_subtest_quoted_text(prefix);
        let parser = create_parser_or_fail(3128 as ::core::ffi::c_int);
        let activation_threshold = -(1 as ::core::ffi::c_int) as size_t as ::core::ffi::c_ulonglong;
        if parser_set_alloc_tracker_activation_threshold(parser, activation_threshold)
            != XML_TRUE as XML_Bool
        {
            fail_test(
                3128 as ::core::ffi::c_int,
                b"check failed: XML_SetAllocTrackerActivationThreshold(parser, (size_t)-1) == XML_TRUE\0",
            );
        }

        let prefix_len = c_string_len(prefix);
        let status = ffi_call4(
            _XML_Parse_SINGLE_BYTES,
            parser,
            prefix,
            prefix_len,
            XML_FALSE as ::core::ffi::c_int,
        );
        if parser_status_is_error(status) {
            xml_failure_for(parser, 3134 as ::core::ffi::c_int);
        }
        if parser_buffer_for(parser, maxbuf - prefix_len).is_null() {
            fail_test(
                3138 as ::core::ffi::c_int,
                b"check failed: XML_GetBuffer(parser, maxbuf - prefix_len) != NULL\0",
            );
        }
        if !parser_buffer_for(parser, maxbuf + 1 as ::core::ffi::c_int).is_null() {
            fail_test(
                3141 as ::core::ffi::c_int,
                b"check failed: XML_GetBuffer(parser, maxbuf + 1) == NULL\0",
            );
        }
        parser_free(parser);
    }
}
extern "C" fn test_getbuffer_allocates_on_zero_len() {
    set_test_info(
        b"test_getbuffer_allocates_on_zero_len\0",
        3147 as ::core::ffi::c_int,
    );

    let mut first_len = 1 as ::core::ffi::c_int;
    while first_len >= 0 as ::core::ffi::c_int {
        set_subtest_with_len_first(first_len);
        let parser = create_parser_or_fail(3151 as ::core::ffi::c_int);

        if parser_buffer_for(parser, first_len).is_null() {
            fail_test(
                3152 as ::core::ffi::c_int,
                b"check failed: XML_GetBuffer(parser, first_len) != NULL\0",
            );
        }
        if parser_buffer_for(parser, 0 as ::core::ffi::c_int).is_null() {
            fail_test(
                3153 as ::core::ffi::c_int,
                b"check failed: XML_GetBuffer(parser, 0) != NULL\0",
            );
        }
        if !matches!(
            parser_parse_buffer_for(
                parser,
                0 as ::core::ffi::c_int,
                XML_FALSE as ::core::ffi::c_int,
            )
                as ::core::ffi::c_uint,
            value if value == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        ) {
            xml_failure_for(parser, 3155 as ::core::ffi::c_int);
        }

        parser_free(parser);
        first_len -= 1;
    }
}
extern "C" fn test_byte_info_at_end() {
    set_test_info(b"test_byte_info_at_end\0", 3162 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(b"<doc></doc>\0");

    if parser_current_byte_index() != -(1 as ::core::ffi::c_int) as XML_Index
        || parser_current_byte_count() != 0 as ::core::ffi::c_int
    {
        fail_test(
            3167 as ::core::ffi::c_int,
            b"Byte index/count incorrect at start of parse\0",
        );
    }

    ensure_parser_success(
        parse_single_bytes(text, c_string_len(text)),
        3170 as ::core::ffi::c_int,
    );

    if parser_current_byte_count() != 0 as ::core::ffi::c_int {
        fail_test(
            3173 as ::core::ffi::c_int,
            b"Terminal byte count incorrect\0",
        );
    }
    if parser_current_byte_index() != c_string_len(text) as XML_Index {
        fail_test(
            3175 as ::core::ffi::c_int,
            b"Terminal byte index incorrect\0",
        );
    }
}
pub const PRE_ERROR_STR: [::core::ffi::c_char; 8] = [
    b'<' as ::core::ffi::c_char,
    b'd' as ::core::ffi::c_char,
    b'o' as ::core::ffi::c_char,
    b'c' as ::core::ffi::c_char,
    b'>' as ::core::ffi::c_char,
    b'<' as ::core::ffi::c_char,
    b'/' as ::core::ffi::c_char,
    0,
];
extern "C" fn test_byte_info_at_error() {
    set_test_info(b"test_byte_info_at_error\0", 3182 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(b"<doc></wombat></doc>\0");

    if !parser_status_is_error(parse_single_bytes(text, c_string_len(text))) {
        fail_test(3187 as ::core::ffi::c_int, b"Syntax error not faulted\0");
    }
    if parser_current_byte_count() != 0 as ::core::ffi::c_int {
        fail_test(3189 as ::core::ffi::c_int, b"Error byte count incorrect\0");
    }
    if parser_current_byte_index() as size_t != c_string_len(PRE_ERROR_STR.as_ptr()) as size_t {
        fail_test(3191 as ::core::ffi::c_int, b"Error byte index incorrect\0");
    }
}
pub const START_ELEMENT: [::core::ffi::c_char; 4] = [
    b'<' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b'>' as ::core::ffi::c_char,
    0,
];
pub const CDATA_TEXT: [::core::ffi::c_char; 6] = [
    b'H' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b'l' as ::core::ffi::c_char,
    b'l' as ::core::ffi::c_char,
    b'o' as ::core::ffi::c_char,
    0,
];
extern "C" fn test_byte_info_at_cdata() {
    set_test_info(b"test_byte_info_at_cdata\0", 3201 as ::core::ffi::c_int);
    let text = bytes_as_c_char_ptr(b"<e>Hello</e>\0");
    let mut offset = 0 as ::core::ffi::c_int;
    let mut size = 0 as ::core::ffi::c_int;
    let mut data = ByteTestData {
        start_element_len: 0,
        cdata_len: 0,
        total_string_len: 0,
    };

    if !parser_input_context(&mut offset, &mut size).is_null() {
        fail_test(
            3208 as ::core::ffi::c_int,
            b"Unexpected context at start of parse\0",
        );
    }

    data.start_element_len = c_string_len(START_ELEMENT.as_ptr());
    data.cdata_len = c_string_len(CDATA_TEXT.as_ptr());
    data.total_string_len = c_string_len(text);
    parser_set_character_data_handler(byte_character_handler_for_tests());
    parser_set_user_data((&mut data as *mut ByteTestData).cast());
    ensure_parser_success(
        parser_parse(text, c_string_len(text), XML_TRUE as ::core::ffi::c_int),
        3216 as ::core::ffi::c_int,
    );
}
extern "C" fn test_predefined_entities() {
    unsafe {
        _check_set_test_info(
            b"test_predefined_entities\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3224 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc>&lt;&gt;&amp;&quot;&apos;</doc>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut expected: *const XML_Char =
            b"<doc>&lt;&gt;&amp;&quot;&apos;</doc>\0".as_ptr() as *const XML_Char;
        let mut result: *const XML_Char = b"<>&\"'\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        XML_SetDefaultHandler(
            g_parser,
            Some(
                accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        CharData_Init(&raw mut storage);
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3238 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        _run_character_check(
            text,
            result,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3244 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_invalid_tag_in_dtd() {
    unsafe {
        _check_set_test_info(
            b"test_invalid_tag_in_dtd\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3256 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM '004-1.ent'>\n<doc></doc>\n\0".as_ptr()
                as *const ::core::ffi::c_char;
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_param
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        _expect_failure(
            text,
            XML_ERROR_EXTERNAL_ENTITY_HANDLING,
            b"Invalid tag IN DTD external param not rejected\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3263 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_not_predefined_entities() {
    unsafe {
        _check_set_test_info(
            b"test_not_predefined_entities\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3268 as ::core::ffi::c_int,
        );
        let mut text: [*const ::core::ffi::c_char; 5] = [
            b"<doc>&pt;</doc>\0".as_ptr() as *const ::core::ffi::c_char,
            b"<doc>&amo;</doc>\0".as_ptr() as *const ::core::ffi::c_char,
            b"<doc>&quid;</doc>\0".as_ptr() as *const ::core::ffi::c_char,
            b"<doc>&apod;</doc>\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ];
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !text[i as usize].is_null() {
            _expect_failure(
                text[i as usize],
                XML_ERROR_UNDEFINED_ENTITY,
                b"Undefined entity not rejected\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3275 as ::core::ffi::c_int,
            );
            XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
            i += 1;
        }
    }
}
extern "C" fn test_ignore_section() {
    unsafe {
        _check_set_test_info(
            b"test_ignore_section\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3283 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM 'foo'>\n<doc><e>&entity;</e></doc>\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut expected: *const XML_Char =
            b"<![IGNORE[<!ELEMENT e (#PCDATA)*>]]>\n&entity;\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_load_ignore
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetDefaultHandler(
            g_parser,
            Some(
                accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetStartDoctypeDeclHandler(
            g_parser,
            Some(
                dummy_start_doctype_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetEndDoctypeDeclHandler(
            g_parser,
            Some(dummy_end_doctype_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        XML_SetElementDeclHandler(
            g_parser,
            Some(
                dummy_element_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut XML_Content,
                    ) -> (),
            ),
        );
        XML_SetStartElementHandler(
            g_parser,
            Some(
                dummy_start_element
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetEndElementHandler(
            g_parser,
            Some(
                dummy_end_element
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3302 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_ignore_section_utf16() {
    unsafe {
        _check_set_test_info(
            b"test_ignore_section_utf16\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3307 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 85] = ::core::mem::transmute::<
            [u8; 85],
            [::core::ffi::c_char; 85],
        >(
            *b"<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0d\0 \0S\0Y\0S\0T\0E\0M\0 \0'\0s\0'\0>\0\n\0<\0d\0>\0<\0e\0>\0&\0e\0n\0;\0<\0/\0e\0>\0<\0/\0d\0>\0\0",
        );
        let mut expected: *const XML_Char =
            b"<![IGNORE[<!ELEMENT e (#PCDATA)*>]]>\n&en;\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_load_ignore_utf16
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetDefaultHandler(
            g_parser,
            Some(
                accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetStartDoctypeDeclHandler(
            g_parser,
            Some(
                dummy_start_doctype_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetEndDoctypeDeclHandler(
            g_parser,
            Some(dummy_end_doctype_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        XML_SetElementDeclHandler(
            g_parser,
            Some(
                dummy_element_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut XML_Content,
                    ) -> (),
            ),
        );
        XML_SetStartElementHandler(
            g_parser,
            Some(
                dummy_start_element
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetEndElementHandler(
            g_parser,
            Some(
                dummy_end_element
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 85]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3329 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_ignore_section_utf16_be() {
    unsafe {
        _check_set_test_info(
            b"test_ignore_section_utf16_be\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3334 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 85] = ::core::mem::transmute::<
            [u8; 85],
            [::core::ffi::c_char; 85],
        >(
            *b"\0<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0d\0 \0S\0Y\0S\0T\0E\0M\0 \0'\0s\0'\0>\0\n\0<\0d\0>\0<\0e\0>\0&\0e\0n\0;\0<\0/\0e\0>\0<\0/\0d\0>\0",
        );
        let mut expected: *const XML_Char =
            b"<![IGNORE[<!ELEMENT e (#PCDATA)*>]]>\n&en;\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_load_ignore_utf16_be
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetDefaultHandler(
            g_parser,
            Some(
                accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetStartDoctypeDeclHandler(
            g_parser,
            Some(
                dummy_start_doctype_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetEndDoctypeDeclHandler(
            g_parser,
            Some(dummy_end_doctype_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        XML_SetElementDeclHandler(
            g_parser,
            Some(
                dummy_element_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut XML_Content,
                    ) -> (),
            ),
        );
        XML_SetStartElementHandler(
            g_parser,
            Some(
                dummy_start_element
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetEndElementHandler(
            g_parser,
            Some(
                dummy_end_element
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 85]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3357 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_bad_ignore_section() {
    unsafe {
        _check_set_test_info(
            b"test_bad_ignore_section\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3363 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM 'foo'>\n<doc><e>&entity;</e></doc>\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut faults: [ExtFaults; 4] = [
            ext_faults {
                parse_text: b"<![IGNORE[<!ELEM\0".as_ptr() as *const ::core::ffi::c_char,
                fail_text: b"Broken-off declaration not faulted\0".as_ptr()
                    as *const ::core::ffi::c_char,
                encoding: ::core::ptr::null::<XML_Char>(),
                error: XML_ERROR_SYNTAX,
            },
            ext_faults {
                parse_text: b"<![IGNORE[\x01]]>\0".as_ptr() as *const ::core::ffi::c_char,
                fail_text: b"Invalid XML character not faulted\0".as_ptr()
                    as *const ::core::ffi::c_char,
                encoding: ::core::ptr::null::<XML_Char>(),
                error: XML_ERROR_INVALID_TOKEN,
            },
            ext_faults {
                parse_text: b"<![IGNORE[\xE2\x82\0".as_ptr() as *const ::core::ffi::c_char,
                fail_text: b"Partial XML character not faulted\0".as_ptr()
                    as *const ::core::ffi::c_char,
                encoding: ::core::ptr::null::<XML_Char>(),
                error: XML_ERROR_PARTIAL_CHAR,
            },
            ext_faults {
                parse_text: ::core::ptr::null::<::core::ffi::c_char>(),
                fail_text: ::core::ptr::null::<::core::ffi::c_char>(),
                encoding: ::core::ptr::null::<XML_Char>(),
                error: XML_ERROR_NONE,
            },
        ];
        let mut fault: *mut ExtFaults = ::core::ptr::null_mut::<ExtFaults>();
        fault = (&raw mut faults as *mut ExtFaults).offset(0 as ::core::ffi::c_int as isize)
            as *mut ExtFaults;
        while !(*fault).parse_text.is_null() {
            set_subtest(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                (*fault).parse_text,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_faulter
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                ),
            );
            XML_SetUserData(g_parser, fault as *mut ::core::ffi::c_void);
            _expect_failure(
                text,
                XML_ERROR_EXTERNAL_ENTITY_HANDLING,
                b"Incomplete IGNORE section not failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3383 as ::core::ffi::c_int,
            );
            XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
            fault = fault.offset(1);
        }
    }
}
extern "C" fn external_bom_checker(
    parser: XML_Parser,
    context: *const XML_Char,
    _base: *const XML_Char,
    system_id: *const XML_Char,
    _public_id: *const XML_Char,
) -> ::core::ffi::c_int {
    let ext_parser = ffi_call3(
        XML_ExternalEntityParserCreate,
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        fail_test(
            3406 as ::core::ffi::c_int,
            b"Could not create external entity parser\0",
        );
    }

    let text = if xml_string_equals(system_id, b"004-2.ent\0") {
        let testdata = mut_from_ptr(parser_user_data_as::<bom_testdata>(parser));
        let external = testdata.external;
        let split = testdata.split;
        testdata.nested_callback_happened = XML_TRUE;
        ensure_parser_success_for(
            ext_parser,
            ffi_call4(
                _XML_Parse_SINGLE_BYTES,
                ext_parser,
                external,
                split,
                XML_FALSE as ::core::ffi::c_int,
            ),
            3417 as ::core::ffi::c_int,
        );
        c_string_offset(external, split)
    } else if xml_string_equals(system_id, b"004-1.ent\0") {
        bytes_as_c_char_ptr(
            b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM '004-2.ent'>\n<!ENTITY % e2 '%e1;'>\n\0",
        )
    } else {
        fail_test(3425 as ::core::ffi::c_int, b"unknown systemId\0");
    };

    ensure_parser_success_for(
        ext_parser,
        ffi_call4(
            _XML_Parse_SINGLE_BYTES,
            ext_parser,
            text,
            c_string_len(text),
            XML_TRUE as ::core::ffi::c_int,
        ),
        3430 as ::core::ffi::c_int,
    );
    parser_free(ext_parser);
    XML_STATUS_OK as ::core::ffi::c_int
}
extern "C" fn test_external_bom_consumed() {
    unsafe {
        _check_set_test_info(
            b"test_external_bom_consumed\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3437 as ::core::ffi::c_int,
        );
        let text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM '004-1.ent'>\n<doc></doc>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let external: *const ::core::ffi::c_char =
            b"\xEF\xBB\xBF<!ATTLIST doc a1 CDATA 'value'>\0".as_ptr() as *const ::core::ffi::c_char;
        let len: ::core::ffi::c_int = strlen(external) as ::core::ffi::c_int;
        let mut split: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while split <= len {
            set_subtest(
                b"split at byte %d\0".as_ptr() as *const ::core::ffi::c_char,
                split,
            );
            let mut testdata: bom_testdata = bom_testdata {
                external: ::core::ptr::null::<::core::ffi::c_char>(),
                split: 0,
                nested_callback_happened: 0,
            };
            testdata.external = external;
            testdata.split = split;
            testdata.nested_callback_happened = XML_FALSE;
            let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
            if parser.is_null() {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    3452 as ::core::ffi::c_int,
                    b"Couldn't create parser\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                parser,
                Some(
                    external_bom_checker
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                ),
            );
            XML_SetUserData(parser, &raw mut testdata as *mut ::core::ffi::c_void);
            if _XML_Parse_SINGLE_BYTES(
                parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _xml_failure(
                    parser,
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    3459 as ::core::ffi::c_int,
                );
            }
            if testdata.nested_callback_happened == 0 {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    3461 as ::core::ffi::c_int,
                    b"ref handler not called\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            XML_ParserFree(parser);
            split += 1;
        }
    }
}
extern "C" fn test_external_entity_values() {
    unsafe {
        _check_set_test_info(
            b"test_external_entity_values\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3469 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM '004-1.ent'>\n<doc></doc>\n\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut data_004_2: [ExtFaults; 12] = [
            ext_faults {
                parse_text: b"<!ATTLIST doc a1 CDATA 'value'>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                fail_text: ::core::ptr::null::<::core::ffi::c_char>(),
                encoding: ::core::ptr::null::<XML_Char>(),
                error: XML_ERROR_NONE,
            },
            ext_faults {
                parse_text: b"<!ATTLIST $doc a1 CDATA 'value'>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                fail_text: b"Invalid token not faulted\0".as_ptr() as *const ::core::ffi::c_char,
                encoding: ::core::ptr::null::<XML_Char>(),
                error: XML_ERROR_INVALID_TOKEN,
            },
            ext_faults {
                parse_text: b"'wombat\0".as_ptr() as *const ::core::ffi::c_char,
                fail_text: b"Unterminated string not faulted\0".as_ptr()
                    as *const ::core::ffi::c_char,
                encoding: ::core::ptr::null::<XML_Char>(),
                error: XML_ERROR_UNCLOSED_TOKEN,
            },
            ext_faults {
                parse_text: b"\xE2\x82\0".as_ptr() as *const ::core::ffi::c_char,
                fail_text: b"Partial UTF-8 character not faulted\0".as_ptr()
                    as *const ::core::ffi::c_char,
                encoding: ::core::ptr::null::<XML_Char>(),
                error: XML_ERROR_PARTIAL_CHAR,
            },
            ext_faults {
                parse_text: b"<?xml version='1.0' encoding='utf-8'?>\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                fail_text: ::core::ptr::null::<::core::ffi::c_char>(),
                encoding: ::core::ptr::null::<XML_Char>(),
                error: XML_ERROR_NONE,
            },
            ext_faults {
                parse_text: b"<?xml?>\0".as_ptr() as *const ::core::ffi::c_char,
                fail_text: b"Malformed XML declaration not faulted\0".as_ptr()
                    as *const ::core::ffi::c_char,
                encoding: ::core::ptr::null::<XML_Char>(),
                error: XML_ERROR_XML_DECL,
            },
            ext_faults {
                parse_text: b"\xEF\xBB\xBF<!ATTLIST doc a1 CDATA 'value'>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                fail_text: ::core::ptr::null::<::core::ffi::c_char>(),
                encoding: ::core::ptr::null::<XML_Char>(),
                error: XML_ERROR_NONE,
            },
            ext_faults {
                parse_text: b"<?xml version='1.0' encoding='utf-8'?>\n$\0".as_ptr()
                    as *const ::core::ffi::c_char,
                fail_text: b"Invalid token after text declaration not faulted\0".as_ptr()
                    as *const ::core::ffi::c_char,
                encoding: ::core::ptr::null::<XML_Char>(),
                error: XML_ERROR_INVALID_TOKEN,
            },
            ext_faults {
                parse_text: b"<?xml version='1.0' encoding='utf-8'?>\n'wombat\0".as_ptr()
                    as *const ::core::ffi::c_char,
                fail_text: b"Unterminated string after text decl not faulted\0".as_ptr()
                    as *const ::core::ffi::c_char,
                encoding: ::core::ptr::null::<XML_Char>(),
                error: XML_ERROR_UNCLOSED_TOKEN,
            },
            ext_faults {
                parse_text: b"<?xml version='1.0' encoding='utf-8'?>\n\xE2\x82\0".as_ptr()
                    as *const ::core::ffi::c_char,
                fail_text: b"Partial UTF-8 character after text decl not faulted\0".as_ptr()
                    as *const ::core::ffi::c_char,
                encoding: ::core::ptr::null::<XML_Char>(),
                error: XML_ERROR_PARTIAL_CHAR,
            },
            ext_faults {
                parse_text: b"%e1;\0".as_ptr() as *const ::core::ffi::c_char,
                fail_text: b"Recursive parameter entity not faulted\0".as_ptr()
                    as *const ::core::ffi::c_char,
                encoding: ::core::ptr::null::<XML_Char>(),
                error: XML_ERROR_RECURSIVE_ENTITY_REF,
            },
            ext_faults {
                parse_text: ::core::ptr::null::<::core::ffi::c_char>(),
                fail_text: ::core::ptr::null::<::core::ffi::c_char>(),
                encoding: ::core::ptr::null::<XML_Char>(),
                error: XML_ERROR_NONE,
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while !data_004_2[i as usize].parse_text.is_null() {
            set_subtest(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                data_004_2[i as usize].parse_text,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_valuer
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                ),
            );
            XML_SetUserData(
                g_parser,
                (&raw mut data_004_2 as *mut ExtFaults).offset(i as isize) as *mut ExtFaults
                    as *mut ::core::ffi::c_void,
            );
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _xml_failure(
                    g_parser,
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    3507 as ::core::ffi::c_int,
                );
            }
            XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
            i += 1;
        }
    }
}
extern "C" fn test_ext_entity_not_standalone() {
    unsafe {
        _check_set_test_info(
            b"test_ext_entity_not_standalone\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3514 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM 'foo'>\n<doc></doc>\0".as_ptr() as *const ::core::ffi::c_char;
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_not_standalone
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        _expect_failure(
            text,
            XML_ERROR_EXTERNAL_ENTITY_HANDLING,
            b"Standalone rejection not caught\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3521 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_ext_entity_value_abort() {
    unsafe {
        _check_set_test_info(
            b"test_ext_entity_value_abort\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3525 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM '004-1.ent'>\n<doc></doc>\n\0".as_ptr()
                as *const ::core::ffi::c_char;
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_value_aborter
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        g_resumable = XML_FALSE;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3534 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_bad_public_doctype() {
    unsafe {
        _check_set_test_info(
            b"test_bad_public_doctype\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3538 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='utf-8'?>\n<!DOCTYPE doc PUBLIC '{BadName}' 'test'>\n<doc></doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        XML_SetDoctypeDeclHandler(
            g_parser,
            Some(
                dummy_start_doctype_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
            Some(dummy_end_doctype_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        _expect_failure(
            text,
            XML_ERROR_PUBLICID,
            b"Bad Public ID not failed\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3546 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_attribute_enum_value() {
    unsafe {
        _check_set_test_info(
            b"test_attribute_enum_value\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3551 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' standalone='no'?>\n<!DOCTYPE animal SYSTEM 'test.dtd'>\n<animal>This is a \n    <a/>  \n\nyellow tiger</animal>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut dtd_data: ExtTest = ExtTest {
            parse_text: b"<!ELEMENT animal (#PCDATA|a)*>\n<!ELEMENT a EMPTY>\n<!ATTLIST animal xml:space (default|preserve) 'preserve'>\0"
                .as_ptr() as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            storage: ::core::ptr::null_mut::<CharData>(),
        };
        let mut expected: *const XML_Char =
            b"This is a \n      \n\nyellow tiger\0".as_ptr() as *const XML_Char;
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_loader
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, &raw mut dtd_data as *mut ::core::ffi::c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetAttlistDeclHandler(
            g_parser,
            Some(
                dummy_attlist_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        _run_ext_character_check(
            text,
            &raw mut dtd_data,
            expected,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3567 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_predefined_entity_redefinition() {
    unsafe {
        _check_set_test_info(
            b"test_predefined_entity_redefinition\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3576 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [\n<!ENTITY apos 'foo'>\n]>\n<doc>&apos;</doc>\0".as_ptr()
                as *const ::core::ffi::c_char;
        _run_character_check(
            text,
            b"'\0".as_ptr() as *const XML_Char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3581 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_dtd_stop_processing() {
    unsafe {
        _check_set_test_info(
            b"test_dtd_stop_processing\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3588 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [\n%foo;\n<!ENTITY bar 'bas'>\n]><doc/>\0".as_ptr()
                as *const ::core::ffi::c_char;
        XML_SetEntityDeclHandler(
            g_parser,
            Some(
                dummy_entity_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                        *const XML_Char,
                        ::core::ffi::c_int,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        init_dummy_handlers();
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3598 as ::core::ffi::c_int,
            );
        }
        if get_dummy_handler_flags() != 0 as ::core::ffi::c_ulong {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3600 as ::core::ffi::c_int,
                b"DTD processing still going after undefined PE\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_public_notation_no_sysid() {
    unsafe {
        _check_set_test_info(
            b"test_public_notation_no_sysid\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3605 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [\n<!NOTATION note PUBLIC 'foo'>\n<!ELEMENT doc EMPTY>\n]>\n<doc/>\0"
                .as_ptr() as *const ::core::ffi::c_char;
        init_dummy_handlers();
        XML_SetNotationDeclHandler(
            g_parser,
            Some(
                dummy_notation_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3615 as ::core::ffi::c_int,
            );
        }
        if get_dummy_handler_flags() != DUMMY_NOTATION_DECL_HANDLER_FLAG {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3617 as ::core::ffi::c_int,
                b"Notation declaration handler not called\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_nested_groups() {
    unsafe {
        _check_set_test_info(
            b"test_nested_groups\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3621 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc (e,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?))))))))))))))))))))))))))))))))>\n<!ELEMENT e EMPTY>]>\n<doc><e/></doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetElementDeclHandler(
            g_parser,
            Some(
                dummy_element_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut XML_Content,
                    ) -> (),
            ),
        );
        XML_SetStartElementHandler(
            g_parser,
            Some(
                record_element_start_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        init_dummy_handlers();
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3641 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, b"doce\0".as_ptr() as *const XML_Char);
        if get_dummy_handler_flags() != DUMMY_ELEMENT_DECL_HANDLER_FLAG {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3644 as ::core::ffi::c_int,
                b"Element handler not fired\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_group_choice() {
    unsafe {
        _check_set_test_info(
            b"test_group_choice\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3648 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc (a|b|c)+>\n<!ELEMENT a EMPTY>\n<!ELEMENT b (#PCDATA)>\n<!ELEMENT c ANY>\n]>\n<doc>\n<a/>\n<b attr='foo'>This is a foo</b>\n<c></c>\n</doc>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
        XML_SetElementDeclHandler(
            g_parser,
            Some(
                dummy_element_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut XML_Content,
                    ) -> (),
            ),
        );
        init_dummy_handlers();
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3665 as ::core::ffi::c_int,
            );
        }
        if get_dummy_handler_flags() != DUMMY_ELEMENT_DECL_HANDLER_FLAG {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3667 as ::core::ffi::c_int,
                b"Element handler flag not raised\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_standalone_parameter_entity() {
    unsafe {
        _check_set_test_info(
            b"test_standalone_parameter_entity\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3671 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' standalone='yes'?>\n<!DOCTYPE doc SYSTEM 'http://example.org/' [\n<!ENTITY % entity '<!ELEMENT doc (#PCDATA)>'>\n%entity;\n]>\n<doc></doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut dtd_data: [::core::ffi::c_char; 22] = ::core::mem::transmute::<
            [u8; 22],
            [::core::ffi::c_char; 22],
        >(*b"<!ENTITY % e1 'foo'>\n\0");
        XML_SetUserData(
            g_parser,
            &raw mut dtd_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        );
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_public
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3685 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_skipped_parameter_entity() {
    unsafe {
        _check_set_test_info(
            b"test_skipped_parameter_entity\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3691 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0'?>\n<!DOCTYPE root SYSTEM 'http://example.org/dtd.ent' [\n<!ELEMENT root (#PCDATA|a)* >\n]>\n<root></root>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut dtd_data: ExtTest = ExtTest {
            parse_text: b"%pe2;\0".as_ptr() as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            storage: ::core::ptr::null_mut::<CharData>(),
        };
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_loader
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, &raw mut dtd_data as *mut ::core::ffi::c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetSkippedEntityHandler(
            g_parser,
            Some(
                dummy_skip_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        init_dummy_handlers();
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3706 as ::core::ffi::c_int,
            );
        }
        if get_dummy_handler_flags() != DUMMY_SKIP_HANDLER_FLAG {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3708 as ::core::ffi::c_int,
                b"Skip handler not executed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_recursive_external_parameter_entity() {
    unsafe {
        _check_set_test_info(
            b"test_recursive_external_parameter_entity\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3713 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0'?>\n<!DOCTYPE root SYSTEM 'http://example.org/dtd.ent' [\n<!ELEMENT root (#PCDATA|a)* >\n]>\n<root></root>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut dtd_data: ExtFaults = ext_faults {
            parse_text: b"<!ENTITY % pe2 '&#37;pe2;'>\n%pe2;\0".as_ptr()
                as *const ::core::ffi::c_char,
            fail_text: b"Recursive external parameter entity not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_RECURSIVE_ENTITY_REF,
        };
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_faulter
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, &raw mut dtd_data as *mut ::core::ffi::c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        _expect_failure(
            text,
            XML_ERROR_EXTERNAL_ENTITY_HANDLING,
            b"Recursive external parameter not spotted\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3727 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_undefined_ext_entity_in_external_dtd() {
    unsafe {
        _check_set_test_info(
            b"test_undefined_ext_entity_in_external_dtd\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3732 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM 'foo'>\n<doc></doc>\n\0".as_ptr() as *const ::core::ffi::c_char;
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_devaluer
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, NULL);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3741 as ::core::ffi::c_int,
            );
        }
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_devaluer
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, g_parser as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3752 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_suspend_xdecl() {
    unsafe {
        _check_set_test_info(
            b"test_suspend_xdecl\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3757 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = long_character_data_text;
        XML_SetXmlDeclHandler(
            g_parser,
            Some(
                entity_suspending_xdecl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, g_parser as *mut ::core::ffi::c_void);
        g_resumable = XML_TRUE;
        if XML_Parse(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3767 as ::core::ffi::c_int,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3769 as ::core::ffi::c_int,
            );
        }
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3773 as ::core::ffi::c_int,
                b"Attempt to parse while suspended not faulted\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3775 as ::core::ffi::c_int,
                b"Suspended parse not faulted with correct error\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_abort_epilog() {
    unsafe {
        _check_set_test_info(
            b"test_abort_epilog\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3780 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc></doc>\n\r\n\0".as_ptr() as *const ::core::ffi::c_char;
        let mut trigger_char: XML_Char = '\r' as i32 as XML_Char;
        XML_SetDefaultHandler(
            g_parser,
            Some(
                selective_aborting_default_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut trigger_char as *mut ::core::ffi::c_void);
        g_resumable = XML_FALSE;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3789 as ::core::ffi::c_int,
                b"Abort not triggered\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_ABORTED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3791 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_abort_epilog_2() {
    unsafe {
        _check_set_test_info(
            b"test_abort_epilog_2\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3796 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc></doc>\n\0".as_ptr() as *const ::core::ffi::c_char;
        let mut trigger_char: XML_Char = '\n' as i32 as XML_Char;
        XML_SetDefaultHandler(
            g_parser,
            Some(
                selective_aborting_default_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut trigger_char as *mut ::core::ffi::c_void);
        g_resumable = XML_FALSE;
        _expect_failure(
            text,
            XML_ERROR_ABORTED,
            b"Abort not triggered\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3803 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_suspend_epilog() {
    unsafe {
        _check_set_test_info(
            b"test_suspend_epilog\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3808 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc></doc>\n\0".as_ptr() as *const ::core::ffi::c_char;
        let mut trigger_char: XML_Char = '\n' as i32 as XML_Char;
        XML_SetDefaultHandler(
            g_parser,
            Some(
                selective_aborting_default_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut trigger_char as *mut ::core::ffi::c_void);
        g_resumable = XML_TRUE;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3817 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_suspend_in_sole_empty_tag() {
    unsafe {
        _check_set_test_info(
            b"test_suspend_in_sole_empty_tag\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3821 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc/>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut rc: XML_Status = XML_STATUS_ERROR;
        XML_SetEndElementHandler(
            g_parser,
            Some(
                suspending_end_handler
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
            ),
        );
        XML_SetUserData(g_parser, g_parser as *mut ::core::ffi::c_void);
        rc = _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        );
        if rc as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3829 as ::core::ffi::c_int,
            );
        } else if rc as ::core::ffi::c_uint
            != XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3831 as ::core::ffi::c_int,
                b"Suspend not triggered\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        rc = XML_ResumeParser(g_parser);
        if rc as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3834 as ::core::ffi::c_int,
            );
        } else if rc as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3836 as ::core::ffi::c_int,
                b"Resume failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_unfinished_epilog() {
    unsafe {
        _check_set_test_info(
            b"test_unfinished_epilog\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3840 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc></doc><\0".as_ptr() as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_UNCLOSED_TOKEN,
            b"Incomplete epilog entry not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3844 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_partial_char_in_epilog() {
    unsafe {
        _check_set_test_info(
            b"test_partial_char_in_epilog\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3848 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc></doc>\xE2\x82\0".as_ptr() as *const ::core::ffi::c_char;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3854 as ::core::ffi::c_int,
            );
        }
        if XML_ParseBuffer(
            g_parser,
            0 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3857 as ::core::ffi::c_int,
                b"Partial character in epilog not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_PARTIAL_CHAR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3859 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_suspend_resume_internal_entity() {
    unsafe {
        _check_set_test_info(
            b"test_suspend_resume_internal_entity\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3864 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ENTITY foo '<suspend>Hi<suspend>Ho</suspend></suspend>'>\n]>\n<doc>&foo;</doc>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut expected1: *const XML_Char = b"Hi\0".as_ptr() as *const XML_Char;
        let mut expected2: *const XML_Char = b"HiHo\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetStartElementHandler(
            g_parser,
            Some(
                start_element_suspender
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if XML_Parse(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3882 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, b"\0".as_ptr() as *const XML_Char);
        if XML_ResumeParser(g_parser) as ::core::ffi::c_uint
            != XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3885 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected1);
        if XML_ResumeParser(g_parser) as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3888 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected2);
    }
}
extern "C" fn test_suspend_resume_internal_entity_issue_629() {
    unsafe {
        _check_set_test_info(
            b"test_suspend_resume_internal_entity_issue_629\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3893 as ::core::ffi::c_int,
        );
        let text: *const ::core::ffi::c_char = b"<!DOCTYPE a [<!ENTITY e '<!--COMMENT-->a'>]><a>&e;<b>\n<aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/></b></a>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let firstChunkSizeBytes: size_t = 54 as size_t;
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        XML_SetUserData(parser, parser as *mut ::core::ffi::c_void);
        XML_SetCommentHandler(
            parser,
            Some(
                suspending_comment_handler
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
            ),
        );
        if XML_Parse(
            parser,
            text,
            firstChunkSizeBytes as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3947 as ::core::ffi::c_int,
            );
        }
        if XML_ResumeParser(parser) as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3949 as ::core::ffi::c_int,
            );
        }
        if _XML_Parse_SINGLE_BYTES(
            parser,
            text.offset(firstChunkSizeBytes as isize),
            strlen(text).wrapping_sub(firstChunkSizeBytes) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3954 as ::core::ffi::c_int,
            );
        }
        XML_ParserFree(parser);
    }
}
extern "C" fn test_resume_entity_with_syntax_error() {
    unsafe {
        _check_set_test_info(
            b"test_resume_entity_with_syntax_error\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3960 as ::core::ffi::c_int,
        );
        if g_chunkSize != 0 as ::core::ffi::c_int {
            return;
        }
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [\n<!ENTITY foo '<suspend>Hi</wombat>'>\n]>\n<doc>&foo;</doc>\n\0"
                .as_ptr() as *const ::core::ffi::c_char;
        XML_SetStartElementHandler(
            g_parser,
            Some(
                start_element_suspender
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        if XML_Parse(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3976 as ::core::ffi::c_int,
            );
        }
        if XML_ResumeParser(g_parser) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3978 as ::core::ffi::c_int,
                b"Syntax error in entity not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_TAG_MISMATCH as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                3980 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_suspend_resume_parameter_entity() {
    unsafe {
        _check_set_test_info(
            b"test_suspend_resume_parameter_entity\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            3985 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ENTITY % foo '<!ELEMENT doc (#PCDATA)*>'>\n%foo;\n]>\n<doc>Hello, world</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut expected: *const XML_Char = b"Hello, world\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetElementDeclHandler(
            g_parser,
            Some(
                element_decl_suspender
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut XML_Content,
                    ) -> (),
            ),
        );
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if XML_Parse(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4001 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, b"\0".as_ptr() as *const XML_Char);
        if XML_ResumeParser(g_parser) as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4004 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_restart_on_error() {
    unsafe {
        _check_set_test_info(
            b"test_restart_on_error\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4010 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<$doc><doc></doc>\0".as_ptr() as *const ::core::ffi::c_char;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4015 as ::core::ffi::c_int,
                b"Invalid tag name not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_INVALID_TOKEN as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4017 as ::core::ffi::c_int,
            );
        }
        if XML_Parse(
            g_parser,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4019 as ::core::ffi::c_int,
                b"Restarting invalid parse not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_INVALID_TOKEN as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4021 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_reject_lt_in_attribute_value() {
    unsafe {
        _check_set_test_info(
            b"test_reject_lt_in_attribute_value\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4026 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [<!ATTLIST doc a CDATA '<bar>'>]>\n<doc></doc>\0".as_ptr()
                as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_INVALID_TOKEN,
            b"Bad attribute default not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4031 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_reject_unfinished_param_in_att_value() {
    unsafe {
        _check_set_test_info(
            b"test_reject_unfinished_param_in_att_value\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4035 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [<!ATTLIST doc a CDATA '&foo'>]>\n<doc></doc>\0".as_ptr()
                as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_INVALID_TOKEN,
            b"Bad attribute default not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4040 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_trailing_cr_in_att_value() {
    unsafe {
        _check_set_test_info(
            b"test_trailing_cr_in_att_value\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4044 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc a='value\r'/>\0".as_ptr() as *const ::core::ffi::c_char;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4049 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_standalone_internal_entity() {
    unsafe {
        _check_set_test_info(
            b"test_standalone_internal_entity\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4056 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' standalone='yes' ?>\n<!DOCTYPE doc [\n  <!ELEMENT doc (#PCDATA)>\n  <!ENTITY % pe '<!ATTLIST doc att2 CDATA \"&ge;\">'>\n  <!ENTITY ge 'AttDefaultValue'>\n  %pe;\n]>\n<doc att2='any'/>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4069 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_skipped_external_entity() {
    unsafe {
        _check_set_test_info(
            b"test_skipped_external_entity\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4074 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM 'http://example.org/'>\n<doc></doc>\n\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut test_data: ExtTest = ExtTest {
            parse_text: b"<!ELEMENT doc EMPTY>\n<!ENTITY % e2 '%e1;'>\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            storage: ::core::ptr::null_mut::<CharData>(),
        };
        XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_loader
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4086 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_skipped_null_loaded_ext_entity() {
    unsafe {
        _check_set_test_info(
            b"test_skipped_null_loaded_ext_entity\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4091 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM 'http://example.org/one.ent'>\n<doc />\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut test_data: ExtHdlrData = ext_hdlr_data {
            parse_text: b"<!ENTITY % pe1 SYSTEM 'http://example.org/two.ent'>\n<!ENTITY % pe2 '%pe1;'>\n%pe2;\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            handler: Some(
                external_entity_null_loader
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
            storage: ::core::ptr::null_mut::<CharData>(),
        };
        XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_oneshot_loader
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4105 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_skipped_unloaded_ext_entity() {
    unsafe {
        _check_set_test_info(
            b"test_skipped_unloaded_ext_entity\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4109 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM 'http://example.org/one.ent'>\n<doc />\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut test_data: ExtHdlrData = ext_hdlr_data {
            parse_text: b"<!ENTITY % pe1 SYSTEM 'http://example.org/two.ent'>\n<!ENTITY % pe2 '%pe1;'>\n%pe2;\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            handler: None,
            storage: ::core::ptr::null_mut::<CharData>(),
        };
        XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_oneshot_loader
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4123 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_param_entity_with_trailing_cr() {
    unsafe {
        _check_set_test_info(
            b"test_param_entity_with_trailing_cr\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4130 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM 'http://example.org/'>\n<doc/>\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut test_data: ExtTest = ExtTest {
            parse_text: b"<!ENTITY % pe '<!ATTLIST doc att CDATA \"default\">\r'>\n%pe;\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            storage: ::core::ptr::null_mut::<CharData>(),
        };
        XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_loader
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetEntityDeclHandler(
            g_parser,
            Some(
                param_entity_match_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                        *const XML_Char,
                        ::core::ffi::c_int,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        param_entity_match_init(
            b"pe\0".as_ptr() as *const XML_Char,
            b"<!ATTLIST doc att CDATA \"default\">\n\0".as_ptr() as *const XML_Char,
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4148 as ::core::ffi::c_int,
            );
        }
        let mut entity_match_flag: ::core::ffi::c_int = get_param_entity_match_flag();
        if entity_match_flag == ENTITY_MATCH_FAIL {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4151 as ::core::ffi::c_int,
                b"Parameter entity CR->NEWLINE conversion failed\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if entity_match_flag == ENTITY_MATCH_NOT_FOUND {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4153 as ::core::ffi::c_int,
                b"Parameter entity not parsed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_invalid_character_entity() {
    unsafe {
        _check_set_test_info(
            b"test_invalid_character_entity\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4159 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [\n  <!ENTITY entity '&#x110000;'>\n]>\n<doc>&entity;</doc>\0".as_ptr()
                as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_BAD_CHAR_REF,
            b"Out of range character reference not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4166 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_invalid_character_entity_2() {
    unsafe {
        _check_set_test_info(
            b"test_invalid_character_entity_2\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4170 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [\n  <!ENTITY entity '&#xg0;'>\n]>\n<doc>&entity;</doc>\0".as_ptr()
                as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_INVALID_TOKEN,
            b"Out of range character reference not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4177 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_invalid_character_entity_3() {
    unsafe {
        _check_set_test_info(
            b"test_invalid_character_entity_3\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4181 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 125] = ::core::mem::transmute::<
            [u8; 125],
            [::core::ffi::c_char; 125],
        >(
            *b"\0<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0d\0o\0c\0 \0[\0\n\0<\0!\0E\0N\0T\0I\0T\0Y\0 \0e\0n\0t\0i\0t\0y\0 \0'\0&\x0E\x04\x0E\x08\0;\0'\0>\0\n\0]\0>\0\n\0<\0d\0o\0c\0>\0&\0e\0n\0t\0i\0t\0y\0;\0<\0/\0d\0o\0c\0>\0",
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 125]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4197 as ::core::ffi::c_int,
                b"Invalid start of entity name not faulted\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_UNDEFINED_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4199 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_invalid_character_entity_4() {
    unsafe {
        _check_set_test_info(
            b"test_invalid_character_entity_4\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4203 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [\n  <!ENTITY entity '&#1114112;'>\n]>\n<doc>&entity;</doc>\0".as_ptr()
                as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_BAD_CHAR_REF,
            b"Out of range character reference not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4210 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_pi_handled_in_default() {
    unsafe {
        _check_set_test_info(
            b"test_pi_handled_in_default\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4215 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<?test processing instruction?>\n<doc/>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut expected: *const XML_Char =
            b"<?test processing instruction?>\n<doc/>\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetDefaultHandler(
            g_parser,
            Some(
                accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4225 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_comment_handled_in_default() {
    unsafe {
        _check_set_test_info(
            b"test_comment_handled_in_default\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4231 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!-- This is a comment -->\n<doc/>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut expected: *const XML_Char =
            b"<!-- This is a comment -->\n<doc/>\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetDefaultHandler(
            g_parser,
            Some(
                accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4241 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_pi_yml() {
    unsafe {
        _check_set_test_info(
            b"test_pi_yml\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4247 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<?yml something like data?><doc/>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut expected: *const XML_Char =
            b"yml: something like data\n\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetProcessingInstructionHandler(
            g_parser,
            Some(
                accumulate_pi_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4257 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_pi_xnl() {
    unsafe {
        _check_set_test_info(
            b"test_pi_xnl\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4262 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<?xnl nothing like data?><doc/>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut expected: *const XML_Char =
            b"xnl: nothing like data\n\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetProcessingInstructionHandler(
            g_parser,
            Some(
                accumulate_pi_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4272 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_pi_xmm() {
    unsafe {
        _check_set_test_info(
            b"test_pi_xmm\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4277 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<?xmm everything like data?><doc/>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut expected: *const XML_Char =
            b"xmm: everything like data\n\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetProcessingInstructionHandler(
            g_parser,
            Some(
                accumulate_pi_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4287 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_utf16_pi() {
    unsafe {
        _check_set_test_info(
            b"test_utf16_pi\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4292 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 21] =
            ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(
                *b"<\0?\0\x04\x0E\x08\x0E?\0>\0<\0q\0/\0>\0\0",
            );
        let mut expected: *const XML_Char =
            b"\xE0\xB8\x84\xE0\xB8\x88: \n\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetProcessingInstructionHandler(
            g_parser,
            Some(
                accumulate_pi_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 21]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4313 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_utf16_be_pi() {
    unsafe {
        _check_set_test_info(
            b"test_utf16_be_pi\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4318 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 21] =
            ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(
                *b"\0<\0?\x0E\x04\x0E\x08\0?\0>\0<\0q\0/\0>\0",
            );
        let mut expected: *const XML_Char =
            b"\xE0\xB8\x84\xE0\xB8\x88: \n\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetProcessingInstructionHandler(
            g_parser,
            Some(
                accumulate_pi_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 21]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4339 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_utf16_be_comment() {
    unsafe {
        _check_set_test_info(
            b"test_utf16_be_comment\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4345 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 51] =
            ::core::mem::transmute::<[u8; 51], [::core::ffi::c_char; 51]>(
                *b"\0<\0!\0-\0-\0 \0C\0o\0m\0m\0e\0n\0t\0 \0A\0 \0-\0-\0>\0\n\0<\0d\0o\0c\0/\0>\0",
            );
        let mut expected: *const XML_Char = b" Comment A \0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetCommentHandler(
            g_parser,
            Some(
                accumulate_comment
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 51]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4359 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_utf16_le_comment() {
    unsafe {
        _check_set_test_info(
            b"test_utf16_le_comment\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4364 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 51] =
            ::core::mem::transmute::<[u8; 51], [::core::ffi::c_char; 51]>(
                *b"<\0!\0-\0-\0 \0C\0o\0m\0m\0e\0n\0t\0 \0B\0 \0-\0-\0>\0\n\0<\0d\0o\0c\0/\0>\0\0",
            );
        let mut expected: *const XML_Char = b" Comment B \0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetCommentHandler(
            g_parser,
            Some(
                accumulate_comment
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 51]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4378 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_missing_encoding_conversion_fn() {
    set_test_info(
        b"test_missing_encoding_conversion_fn\0",
        4386 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(b"<?xml version='1.0' encoding='no-conv'?>\n<doc>\x81</doc>\0");

    parser_set_unknown_encoding_handler(misc_encoding_handler_for_tests(), NULL);
    expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Encoding with missing convert() not faulted\0",
        4398 as ::core::ffi::c_int,
    );
}

extern "C" fn test_failing_encoding_conversion_fn() {
    set_test_info(
        b"test_failing_encoding_conversion_fn\0",
        4402 as ::core::ffi::c_int,
    );
    let text =
        bytes_as_c_char_ptr(b"<?xml version='1.0' encoding='failing-conv'?>\n<doc>\x81</doc>\0");

    parser_set_unknown_encoding_handler(misc_encoding_handler_for_tests(), NULL);
    expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Encoding with failing convert() not faulted\0",
        4413 as ::core::ffi::c_int,
    );
}

extern "C" fn test_unknown_encoding_success() {
    set_test_info(
        b"test_unknown_encoding_success\0",
        4418 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='prefix-conv'?>\n<\x81d\x80oc>Hello, world</\x81d\x80oc>\0",
    );

    parser_set_unknown_encoding_handler(misc_encoding_handler_for_tests(), NULL);
    run_character_check(
        text,
        bytes_as_xml_char_ptr(b"Hello, world\0"),
        4424 as ::core::ffi::c_int,
    );
}

extern "C" fn test_unknown_encoding_bad_name() {
    set_test_info(
        b"test_unknown_encoding_bad_name\0",
        4429 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='prefix-conv'?>\n<\xFFdoc>Hello, world</\xFFdoc>\0",
    );

    parser_set_unknown_encoding_handler(misc_encoding_handler_for_tests(), NULL);
    expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Bad name start in unknown encoding not faulted\0",
        4435 as ::core::ffi::c_int,
    );
}

extern "C" fn test_unknown_encoding_bad_name_2() {
    set_test_info(
        b"test_unknown_encoding_bad_name_2\0",
        4440 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='prefix-conv'?>\n<d\xFFoc>Hello, world</d\xFFoc>\0",
    );

    parser_set_unknown_encoding_handler(misc_encoding_handler_for_tests(), NULL);
    expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Bad name in unknown encoding not faulted\0",
        4446 as ::core::ffi::c_int,
    );
}

extern "C" fn test_unknown_encoding_long_name_1() {
    set_test_info(
        b"test_unknown_encoding_long_name_1\0",
        4453 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='prefix-conv'?>\n<abcdefghabcdefghabcdefghijkl\x80m\x80n\x80o\x80p>Hi</abcdefghabcdefghabcdefghijkl\x80m\x80n\x80o\x80p>\0",
    );
    let expected = bytes_as_xml_char_ptr(b"abcdefghabcdefghabcdefghijklmnop\0");
    let mut storage = CharData {
        count: 0,
        data: [0; 2048],
    };

    char_data_init(&mut storage);
    parser_set_unknown_encoding_handler(misc_encoding_handler_for_tests(), NULL);
    parser_set_start_element_handler(record_element_start_handler_for_tests());
    parser_set_user_data((&raw mut storage).cast());
    ensure_parser_success(
        parse_single_bytes_c_string(text),
        4467 as ::core::ffi::c_int,
    );
    char_data_check_xml_chars(&mut storage, expected);
}

extern "C" fn test_unknown_encoding_long_name_2() {
    set_test_info(
        b"test_unknown_encoding_long_name_2\0",
        4475 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='prefix-conv'?>\n<abcdefghabcdefghabcdefghijklmnop>Hi</abcdefghabcdefghabcdefghijklmnop>\0",
    );
    let expected = bytes_as_xml_char_ptr(b"abcdefghabcdefghabcdefghijklmnop\0");
    let mut storage = CharData {
        count: 0,
        data: [0; 2048],
    };

    char_data_init(&mut storage);
    parser_set_unknown_encoding_handler(misc_encoding_handler_for_tests(), NULL);
    parser_set_start_element_handler(record_element_start_handler_for_tests());
    parser_set_user_data((&raw mut storage).cast());
    ensure_parser_success(
        parse_single_bytes_c_string(text),
        4489 as ::core::ffi::c_int,
    );
    char_data_check_xml_chars(&mut storage, expected);
}
extern "C" fn test_invalid_unknown_encoding() {
    set_test_info(
        b"test_invalid_unknown_encoding\0",
        4494 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='invalid-9'?>\n<doc>Hello world</doc>\0",
    );

    parser_set_unknown_encoding_handler(misc_encoding_handler_for_tests(), NULL);
    expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Invalid unknown encoding not faulted\0",
        4500 as ::core::ffi::c_int,
    );
}

extern "C" fn test_unknown_ascii_encoding_ok() {
    set_test_info(
        b"test_unknown_ascii_encoding_ok\0",
        4504 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='ascii-like'?>\n<doc>Hello, world</doc>\0",
    );

    parser_set_unknown_encoding_handler(misc_encoding_handler_for_tests(), NULL);
    run_character_check(
        text,
        bytes_as_xml_char_ptr(b"Hello, world\0"),
        4509 as ::core::ffi::c_int,
    );
}

extern "C" fn test_unknown_ascii_encoding_fail() {
    set_test_info(
        b"test_unknown_ascii_encoding_fail\0",
        4513 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='ascii-like'?>\n<doc>Hello, \x80 world</doc>\0",
    );

    parser_set_unknown_encoding_handler(misc_encoding_handler_for_tests(), NULL);
    expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid character not faulted\0",
        4519 as ::core::ffi::c_int,
    );
}

extern "C" fn test_unknown_encoding_invalid_length() {
    set_test_info(
        b"test_unknown_encoding_invalid_length\0",
        4523 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='invalid-len'?>\n<doc>Hello, world</doc>\0",
    );

    parser_set_unknown_encoding_handler(misc_encoding_handler_for_tests(), NULL);
    expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Invalid unknown encoding not faulted\0",
        4529 as ::core::ffi::c_int,
    );
}

extern "C" fn test_unknown_encoding_invalid_topbit() {
    set_test_info(
        b"test_unknown_encoding_invalid_topbit\0",
        4533 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='invalid-a'?>\n<doc>Hello, world</doc>\0",
    );

    parser_set_unknown_encoding_handler(misc_encoding_handler_for_tests(), NULL);
    expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Invalid unknown encoding not faulted\0",
        4539 as ::core::ffi::c_int,
    );
}

extern "C" fn test_unknown_encoding_invalid_surrogate() {
    set_test_info(
        b"test_unknown_encoding_invalid_surrogate\0",
        4543 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='invalid-surrogate'?>\n<doc>Hello, \x82 world</doc>\0",
    );

    parser_set_unknown_encoding_handler(misc_encoding_handler_for_tests(), NULL);
    expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid unknown encoding not faulted\0",
        4549 as ::core::ffi::c_int,
    );
}

extern "C" fn test_unknown_encoding_invalid_high() {
    set_test_info(
        b"test_unknown_encoding_invalid_high\0",
        4553 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='invalid-high'?>\n<doc>Hello, world</doc>\0",
    );

    parser_set_unknown_encoding_handler(misc_encoding_handler_for_tests(), NULL);
    expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Invalid unknown encoding not faulted\0",
        4559 as ::core::ffi::c_int,
    );
}

extern "C" fn test_unknown_encoding_invalid_attr_value() {
    set_test_info(
        b"test_unknown_encoding_invalid_attr_value\0",
        4563 as ::core::ffi::c_int,
    );
    let text =
        bytes_as_c_char_ptr(b"<?xml version='1.0' encoding='prefix-conv'?>\n<doc attr='\xFF0'/>\0");

    parser_set_unknown_encoding_handler(misc_encoding_handler_for_tests(), NULL);
    expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid attribute valid not faulted\0",
        4569 as ::core::ffi::c_int,
    );
}

extern "C" fn test_unknown_encoding_user_data_primary() {
    set_test_info(
        b"test_unknown_encoding_user_data_primary\0",
        4573 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(b"<?xml version='1.0' encoding='x-unk'?>\n<root />\n\0");
    let parser = parser_create();

    parser_set_unknown_encoding_handler_for(
        parser,
        user_data_checking_unknown_encoding_handler_for_tests(),
        0xc0ffee as ::core::ffi::c_int as intptr_t as *mut ::core::ffi::c_void,
    );
    assert_status_ok(
        parse_single_bytes_c_string_for(parser, text),
        4583 as ::core::ffi::c_int,
        b"check failed: _XML_Parse_SINGLE_BYTES(parser, text, (int)strlen(text), XML_TRUE) == XML_STATUS_OK\0",
    );
    parser_free(parser);
}

extern "C" fn test_unknown_encoding_user_data_secondary() {
    set_test_info(
        b"test_unknown_encoding_user_data_secondary\0",
        4589 as ::core::ffi::c_int,
    );
    let text_main = bytes_as_c_char_ptr(
        b"<!DOCTYPE r [\n  <!ENTITY ext SYSTEM 'ext.ent'>\n]>\n<r>&ext;</r>\n\0",
    );
    let text_external =
        bytes_as_c_char_ptr(b"<?xml version='1.0' encoding='x-unk'?>\n<e>data</e>\0");
    let mut test_data = ExtTest2 {
        parse_text: text_external,
        parse_len: c_string_len(text_external),
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    let parser = parser_create();

    parser_set_external_entity_ref_handler_for(parser, external_entity_loader2_handler_for_tests());
    parser_set_unknown_encoding_handler_for(
        parser,
        user_data_checking_unknown_encoding_handler_for_tests(),
        0xc0ffee as ::core::ffi::c_int as intptr_t as *mut ::core::ffi::c_void,
    );
    parser_set_user_data_for(parser, (&raw mut test_data).cast());
    assert_status_ok(
        parse_single_bytes_c_string_for(parser, text_main),
        4607 as ::core::ffi::c_int,
        b"check failed: _XML_Parse_SINGLE_BYTES(parser, text_main, (int)strlen(text_main), XML_TRUE) == XML_STATUS_OK\0",
    );
    parser_free(parser);
}
extern "C" fn test_ext_entity_latin1_utf16le_bom() {
    unsafe {
        _check_set_test_info(
            b"test_ext_entity_latin1_utf16le_bom\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4617 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut test_data: ExtTest2 = ExtTest2 {
            parse_text: b"\xFF\xFEL \0".as_ptr() as *const ::core::ffi::c_char,
            parse_len: 4 as ::core::ffi::c_int,
            encoding: b"iso-8859-1\0".as_ptr() as *const XML_Char,
            storage: ::core::ptr::null_mut::<CharData>(),
        };
        let mut expected: *const XML_Char = b"\xC3\xBF\xC3\xBEL \0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        test_data.storage = &raw mut storage;
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_loader2
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                ext2_accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4643 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_ext_entity_latin1_utf16be_bom() {
    unsafe {
        _check_set_test_info(
            b"test_ext_entity_latin1_utf16be_bom\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4648 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut test_data: ExtTest2 = ExtTest2 {
            parse_text: b"\xFE\xFF L\0".as_ptr() as *const ::core::ffi::c_char,
            parse_len: 4 as ::core::ffi::c_int,
            encoding: b"iso-8859-1\0".as_ptr() as *const XML_Char,
            storage: ::core::ptr::null_mut::<CharData>(),
        };
        let mut expected: *const XML_Char = b"\xC3\xBE\xC3\xBF L\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        test_data.storage = &raw mut storage;
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_loader2
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                ext2_accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4674 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_ext_entity_latin1_utf16le_bom2() {
    unsafe {
        _check_set_test_info(
            b"test_ext_entity_latin1_utf16le_bom2\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4683 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut test_data: ExtTest2 = ExtTest2 {
            parse_text: b"\xFF\xFEL \0".as_ptr() as *const ::core::ffi::c_char,
            parse_len: 4 as ::core::ffi::c_int,
            encoding: b"iso-8859-1\0".as_ptr() as *const XML_Char,
            storage: ::core::ptr::null_mut::<CharData>(),
        };
        let mut expected: *const XML_Char = b"\xC3\xBF\xC3\xBEL \0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        test_data.storage = &raw mut storage;
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_loader2
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                ext2_accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4709 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_ext_entity_latin1_utf16be_bom2() {
    unsafe {
        _check_set_test_info(
            b"test_ext_entity_latin1_utf16be_bom2\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4714 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut test_data: ExtTest2 = ExtTest2 {
            parse_text: b"\xFE\xFF L\0".as_ptr() as *const ::core::ffi::c_char,
            parse_len: 4 as ::core::ffi::c_int,
            encoding: b"iso-8859-1\0".as_ptr() as *const XML_Char,
            storage: ::core::ptr::null_mut::<CharData>(),
        };
        let mut expected: *const XML_Char = b"\xC3\xBE\xC3\xBF L\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        test_data.storage = &raw mut storage;
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_loader2
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                ext2_accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4740 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_ext_entity_utf16_be() {
    unsafe {
        _check_set_test_info(
            b"test_ext_entity_utf16_be\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4746 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut test_data: ExtTest2 = ExtTest2 {
            parse_text: b"<\0e\0/\0>\0\0".as_ptr() as *const ::core::ffi::c_char,
            parse_len: 8 as ::core::ffi::c_int,
            encoding: b"utf-16be\0".as_ptr() as *const XML_Char,
            storage: ::core::ptr::null_mut::<CharData>(),
        };
        let mut expected: *const XML_Char =
            b"\xE3\xB0\x80\xE6\x94\x80\xE2\xBC\x80\xE3\xB8\x80\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        test_data.storage = &raw mut storage;
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_loader2
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                ext2_accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4769 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_ext_entity_utf16_le() {
    unsafe {
        _check_set_test_info(
            b"test_ext_entity_utf16_le\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4775 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut test_data: ExtTest2 = ExtTest2 {
            parse_text: b"\0<\0e\0/\0>\0".as_ptr() as *const ::core::ffi::c_char,
            parse_len: 8 as ::core::ffi::c_int,
            encoding: b"utf-16le\0".as_ptr() as *const XML_Char,
            storage: ::core::ptr::null_mut::<CharData>(),
        };
        let mut expected: *const XML_Char =
            b"\xE3\xB0\x80\xE6\x94\x80\xE2\xBC\x80\xE3\xB8\x80\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        test_data.storage = &raw mut storage;
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_loader2
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                ext2_accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4798 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_ext_entity_utf16_unknown() {
    unsafe {
        _check_set_test_info(
            b"test_ext_entity_utf16_unknown\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4810 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut test_data: ExtFaults2 = ExtFaults2 {
            parse_text: b"a\0b\0c\0\0".as_ptr() as *const ::core::ffi::c_char,
            parse_len: 6 as ::core::ffi::c_int,
            fail_text: b"Invalid character in entity not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_INVALID_TOKEN,
        };
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_faulter2
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
        _expect_failure(
            text,
            XML_ERROR_EXTERNAL_ENTITY_HANDLING,
            b"Invalid character should not have been accepted\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4822 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_ext_entity_utf8_non_bom() {
    unsafe {
        _check_set_test_info(
            b"test_ext_entity_utf8_non_bom\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4827 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut test_data: ExtTest2 = ExtTest2 {
            parse_text: b"\xEF\xBB\x80\0".as_ptr() as *const ::core::ffi::c_char,
            parse_len: 3 as ::core::ffi::c_int,
            encoding: ::core::ptr::null::<XML_Char>(),
            storage: ::core::ptr::null_mut::<CharData>(),
        };
        let mut expected: *const XML_Char = b"\xEF\xBB\x80\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        test_data.storage = &raw mut storage;
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_loader2
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                ext2_accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4849 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_utf8_in_cdata_section() {
    unsafe {
        _check_set_test_info(
            b"test_utf8_in_cdata_section\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4855 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc><![CDATA[one \xC3\xA9 two]]></doc>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut expected: *const XML_Char = b"one \xC3\xA9 two\0".as_ptr() as *const XML_Char;
        _run_character_check(
            text,
            expected,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4863 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_utf8_in_cdata_section_2() {
    unsafe {
        _check_set_test_info(
            b"test_utf8_in_cdata_section_2\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4868 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc><![CDATA[\xC3\xA9]\xC3\xA9two]]></doc>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut expected: *const XML_Char = b"\xC3\xA9]\xC3\xA9two\0".as_ptr() as *const XML_Char;
        _run_character_check(
            text,
            expected,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4876 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_utf8_in_start_tags() {
    unsafe {
        _check_set_test_info(
            b"test_utf8_in_start_tags\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4880 as ::core::ffi::c_int,
        );
        let mut cases: [test_case; 24] = [
            test_case {
                goodName: true_0 != 0,
                goodNameStart: true_0 != 0,
                tagName: b":\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xBA\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: true_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"9\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xB9\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: true_0 != 0,
                goodNameStart: true_0 != 0,
                tagName: b"\xDB\xA5\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\x9B\xA5\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xDB%\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xDB\xE5\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: true_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xCC\x81\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\x8C\x81\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xCC\x01\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xCC\xC1\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: true_0 != 0,
                goodNameStart: true_0 != 0,
                tagName: b"\xE0\xA4\x85\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xA0\xA4\x85\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xE0$\x85\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xE0\xE4\x85\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xE0\xA4\x05\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xE0\xA4\xC5\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: true_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xE0\xA4\x81\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xA0\xA4\x81\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xE0$\x81\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xE0\xE4\x81\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xE0\xA4\x01\0".as_ptr() as *const ::core::ffi::c_char,
            },
            test_case {
                goodName: false_0 != 0,
                goodNameStart: false_0 != 0,
                tagName: b"\xE0\xA4\xC1\0".as_ptr() as *const ::core::ffi::c_char,
            },
        ];
        let atNameStart: [bool; 2] = [true_0 != 0, false_0 != 0];
        let mut i: size_t = 0 as size_t;
        let mut doc: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut failCount: size_t = 0 as size_t;
        if g_reparseDeferralEnabledDefault != 0 {
            return;
        }
        while i
            < (::core::mem::size_of::<[test_case; 24]>() as usize)
                .wrapping_div(::core::mem::size_of::<test_case>() as usize)
        {
            let mut j: size_t = 0 as size_t;
            while j
                < (::core::mem::size_of::<[bool; 2]>() as usize)
                    .wrapping_div(::core::mem::size_of::<bool>() as usize)
            {
                let expectedSuccess: bool = if atNameStart[j as usize] as ::core::ffi::c_int != 0 {
                    cases[i as usize].goodNameStart as ::core::ffi::c_int
                } else {
                    cases[i as usize].goodName as ::core::ffi::c_int
                } != 0;
                snprintf(
                    &raw mut doc as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
                    b"<%s%s><!--\0".as_ptr() as *const ::core::ffi::c_char,
                    if atNameStart[j as usize] as ::core::ffi::c_int != 0 {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"a\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    cases[i as usize].tagName,
                );
                let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
                let status: XML_Status = _XML_Parse_SINGLE_BYTES(
                    parser,
                    &raw mut doc as *mut ::core::ffi::c_char,
                    strlen(&raw mut doc as *mut ::core::ffi::c_char) as ::core::ffi::c_int,
                    XML_FALSE as ::core::ffi::c_int,
                ) as XML_Status;
                let mut success: bool = true_0 != 0;
                if (status as ::core::ffi::c_uint
                    == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
                    as ::core::ffi::c_int
                    != expectedSuccess as ::core::ffi::c_int
                {
                    success = false_0 != 0;
                }
                if status as ::core::ffi::c_uint
                    == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
                    && XML_GetErrorCode(parser) as ::core::ffi::c_uint
                        != XML_ERROR_INVALID_TOKEN as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    success = false_0 != 0;
                }
                if !success {
                    fprintf(
                        stderr,
                        b"FAIL case %2u (%sat name start, %u-byte sequence, error code %d)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        (i as ::core::ffi::c_uint).wrapping_add(1 as ::core::ffi::c_uint),
                        if atNameStart[j as usize] as ::core::ffi::c_int != 0 {
                            b"    \0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"not \0".as_ptr() as *const ::core::ffi::c_char
                        },
                        strlen(cases[i as usize].tagName) as ::core::ffi::c_uint,
                        XML_GetErrorCode(parser) as ::core::ffi::c_uint,
                    );
                    failCount = failCount.wrapping_add(1);
                }
                XML_ParserFree(parser);
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        if failCount > 0 as size_t {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4981 as ::core::ffi::c_int,
                b"UTF-8 regression detected\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_trailing_spaces_in_elements() {
    unsafe {
        _check_set_test_info(
            b"test_trailing_spaces_in_elements\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            4987 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc   >Hi</doc >\0".as_ptr() as *const ::core::ffi::c_char;
        let mut expected: *const XML_Char = b"doc/doc\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetElementHandler(
            g_parser,
            Some(
                record_element_start_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
            Some(
                record_element_end_handler
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                4998 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_utf16_attribute() {
    unsafe {
        _check_set_test_info(
            b"test_utf16_attribute\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5003 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 23] =
            ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(
                *b"<\0d\0 \0\x04\x0E\x08\x0E=\0'\0a\0'\0/\0>\0\0",
            );
        let mut expected: *const XML_Char = b"a\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetStartElementHandler(
            g_parser,
            Some(
                accumulate_attribute
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 23]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5018 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_utf16_second_attr() {
    unsafe {
        _check_set_test_info(
            b"test_utf16_second_attr\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5023 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 35] =
            ::core::mem::transmute::<[u8; 35], [::core::ffi::c_char; 35]>(
                *b"<\0d\0 \0a\0=\0'\x001\0'\0 \0\x04\x0E\x08\x0E=\0'\x002\0'\0/\0>\0\0",
            );
        let mut expected: *const XML_Char = b"1\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetStartElementHandler(
            g_parser,
            Some(
                accumulate_attribute
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 35]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5038 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_attr_after_solidus() {
    unsafe {
        _check_set_test_info(
            b"test_attr_after_solidus\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5043 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc attr1='a' / attr2='b'>\0".as_ptr() as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_INVALID_TOKEN,
            b"Misplaced / not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5046 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_utf16_pe() {
    unsafe {
        _check_set_test_info(
            b"test_utf16_pe\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5050 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 155] = ::core::mem::transmute::<
            [u8; 155],
            [::core::ffi::c_char; 155],
        >(
            *b"\0<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0d\0o\0c\0 \0[\0\n\0<\0!\0E\0N\0T\0I\0T\0Y\0 \0%\0 \x0E\x04\x0E\x08\0 \0'\0<\0!\0E\0L\0E\0M\0E\0N\0T\0 \0d\0o\0c\0 \0(\0#\0P\0C\0D\0A\0T\0A\0)\0>\0'\0>\0\n\0%\x0E\x04\x0E\x08\0;\0\n\0]\0>\0\n\0<\0d\0o\0c\0>\0<\0/\0d\0o\0c\0>\0",
        );
        let mut expected: *const XML_Char =
            b"\xE0\xB8\x84\xE0\xB8\x88=<!ELEMENT doc (#PCDATA)>\n\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        XML_SetEntityDeclHandler(
            g_parser,
            Some(
                accumulate_entity_decl
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                        *const XML_Char,
                        ::core::ffi::c_int,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 155]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5080 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_bad_attr_desc_keyword() {
    unsafe {
        _check_set_test_info(
            b"test_bad_attr_desc_keyword\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5086 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [\n  <!ATTLIST doc attr CDATA #!IMPLIED>\n]>\n<doc />\0".as_ptr()
                as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_INVALID_TOKEN,
            b"Bad keyword !IMPLIED not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5093 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_bad_attr_desc_keyword_utf16() {
    unsafe {
        _check_set_test_info(
            b"test_bad_attr_desc_keyword_utf16\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5101 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 91] = ::core::mem::transmute::<
            [u8; 91],
            [::core::ffi::c_char; 91],
        >(
            *b"\0<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0d\0 \0[\0\n\0<\0!\0A\0T\0T\0L\0I\0S\0T\0 \0d\0 \0a\0 \0C\0D\0A\0T\0A\0 \0#\x0E\x04\x0E\x08\0>\0\n\0]\0>\0<\0d\0/\0>\0",
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 91]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5117 as ::core::ffi::c_int,
                b"Invalid UTF16 attribute keyword not faulted\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_SYNTAX as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5119 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_bad_doctype() {
    unsafe {
        _check_set_test_info(
            b"test_bad_doctype\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5126 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<?xml version='1.0' encoding='prefix-conv'?>\n<!DOCTYPE doc [ \x80D ]><doc/>\0"
                .as_ptr() as *const ::core::ffi::c_char;
        XML_SetUnknownEncodingHandler(
            g_parser,
            Some(
                MiscEncodingHandler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut XML_Encoding,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
        );
        _expect_failure(
            text,
            XML_ERROR_SYNTAX,
            b"Invalid bytes in DOCTYPE not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5132 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_bad_doctype_utf8() {
    unsafe {
        _check_set_test_info(
            b"test_bad_doctype_utf8\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5136 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE \xDB%doc><doc/>\0".as_ptr() as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_INVALID_TOKEN,
            b"Invalid UTF-8 in DOCTYPE not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5140 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_bad_doctype_utf16() {
    unsafe {
        _check_set_test_info(
            b"test_bad_doctype_utf16\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5144 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 53] = ::core::mem::transmute::<
            [u8; 53],
            [::core::ffi::c_char; 53],
        >(
            *b"\0<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0d\0o\0c\0 \0[\0 \x06\xF2\0 \0]\0>\0<\0d\0o\0c\0/\0>\0",
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 53]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5157 as ::core::ffi::c_int,
                b"Invalid bytes in DOCTYPE not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_SYNTAX as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5159 as ::core::ffi::c_int,
            );
        }
    }
}
extern "C" fn test_bad_doctype_plus() {
    unsafe {
        _check_set_test_info(
            b"test_bad_doctype_plus\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5163 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE 1+ [ <!ENTITY foo 'bar'> ]>\n<1+>&foo;</1+>\0".as_ptr()
                as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_INVALID_TOKEN,
            b"'+' in document name not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5168 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_bad_doctype_star() {
    unsafe {
        _check_set_test_info(
            b"test_bad_doctype_star\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5172 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE 1* [ <!ENTITY foo 'bar'> ]>\n<1*>&foo;</1*>\0".as_ptr()
                as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_INVALID_TOKEN,
            b"'*' in document name not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5177 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_bad_doctype_query() {
    unsafe {
        _check_set_test_info(
            b"test_bad_doctype_query\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5181 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE 1? [ <!ENTITY foo 'bar'> ]>\n<1?>&foo;</1?>\0".as_ptr()
                as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_INVALID_TOKEN,
            b"'?' in document name not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5186 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_unknown_encoding_bad_ignore() {
    set_test_info(
        b"test_unknown_encoding_bad_ignore\0",
        5190 as ::core::ffi::c_int,
    );
    let text = bytes_as_c_char_ptr(
        b"<?xml version='1.0' encoding='prefix-conv'?><!DOCTYPE doc SYSTEM 'foo'><doc><e>&entity;</e></doc>\0",
    );
    let mut fault = ExtFaults {
        parse_text: bytes_as_c_char_ptr(b"<![IGNORE[<!ELEMENT \xFFG (#PCDATA)*>]]>\0"),
        fail_text: bytes_as_c_char_ptr(b"Invalid character not faulted\0"),
        encoding: bytes_as_xml_char_ptr(b"prefix-conv\0"),
        error: XML_ERROR_INVALID_TOKEN,
    };

    parser_set_unknown_encoding_handler(misc_encoding_handler_for_tests(), NULL);
    parser_set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
    parser_set_external_entity_ref_handler(external_entity_faulter_handler_for_tests());
    parser_set_user_data((&raw mut fault).cast());
    expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Bad IGNORE section with unknown encoding not failed\0",
        5203 as ::core::ffi::c_int,
    );
}
extern "C" fn test_entity_in_utf16_be_attr() {
    unsafe {
        _check_set_test_info(
            b"test_entity_in_utf16_be_attr\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5207 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 55] = ::core::mem::transmute::<
            [u8; 55],
            [::core::ffi::c_char; 55],
        >(
            *b"\0<\0e\0 \0a\0=\0'\0&\0#\x002\x002\08\0;\0 \0&\0#\0x\x000\x000\0E\x004\0;\0'\0>\0<\0/\0e\0>\0",
        );
        let mut expected: *const XML_Char = b"\xC3\xA4 \xC3\xA4\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        XML_SetStartElementHandler(
            g_parser,
            Some(
                accumulate_attribute
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 55]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5224 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_entity_in_utf16_le_attr() {
    unsafe {
        _check_set_test_info(
            b"test_entity_in_utf16_le_attr\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5229 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 55] = ::core::mem::transmute::<
            [u8; 55],
            [::core::ffi::c_char; 55],
        >(
            *b"<\0e\0 \0a\0=\0'\0&\0#\x002\x002\08\0;\0 \0&\0#\0x\x000\x000\0E\x004\0;\0'\0>\0<\0/\0e\0>\0\0",
        );
        let mut expected: *const XML_Char = b"\xC3\xA4 \xC3\xA4\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        XML_SetStartElementHandler(
            g_parser,
            Some(
                accumulate_attribute
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 55]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5246 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_entity_public_utf16_be() {
    unsafe {
        _check_set_test_info(
            b"test_entity_public_utf16_be\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5251 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 137] = ::core::mem::transmute::<
            [u8; 137],
            [::core::ffi::c_char; 137],
        >(
            *b"\0<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0d\0 \0[\0\n\0<\0!\0E\0N\0T\0I\0T\0Y\0 \0%\0 \0e\0 \0P\0U\0B\0L\0I\0C\0 \0'\0f\0o\0o\0'\0 \0'\0b\0a\0r\0.\0e\0n\0t\0'\0>\0\n\0%\0e\0;\0\n\0]\0>\0\n\0<\0d\0>\0&\0j\0;\0<\0/\0d\0>\0",
        );
        let mut test_data: ExtTest2 = ExtTest2 {
            parse_text: b"\0<\0!\0E\0N\0T\0I\0T\0Y\0 \0j\0 \0'\0b\0a\0z\0'\0>\0".as_ptr()
                as *const ::core::ffi::c_char,
            parse_len: 34 as ::core::ffi::c_int,
            encoding: ::core::ptr::null::<XML_Char>(),
            storage: ::core::ptr::null_mut::<CharData>(),
        };
        let mut expected: *const XML_Char = b"baz\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        test_data.storage = &raw mut storage;
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_loader2
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                ext2_accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 137]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5278 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_entity_public_utf16_le() {
    unsafe {
        _check_set_test_info(
            b"test_entity_public_utf16_le\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5283 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 137] = ::core::mem::transmute::<
            [u8; 137],
            [::core::ffi::c_char; 137],
        >(
            *b"<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0d\0 \0[\0\n\0<\0!\0E\0N\0T\0I\0T\0Y\0 \0%\0 \0e\0 \0P\0U\0B\0L\0I\0C\0 \0'\0f\0o\0o\0'\0 \0'\0b\0a\0r\0.\0e\0n\0t\0'\0>\0\n\0%\0e\0;\0\n\0]\0>\0\n\0<\0d\0>\0&\0j\0;\0<\0/\0d\0>\0\0",
        );
        let mut test_data: ExtTest2 = ExtTest2 {
            parse_text: b"<\0!\0E\0N\0T\0I\0T\0Y\0 \0j\0 \0'\0b\0a\0z\0'\0>\0\0".as_ptr()
                as *const ::core::ffi::c_char,
            parse_len: 34 as ::core::ffi::c_int,
            encoding: ::core::ptr::null::<XML_Char>(),
            storage: ::core::ptr::null_mut::<CharData>(),
        };
        let mut expected: *const XML_Char = b"baz\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        test_data.storage = &raw mut storage;
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_loader2
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                ext2_accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 137]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5310 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_short_doctype() {
    unsafe {
        _check_set_test_info(
            b"test_short_doctype\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5318 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc></doc>\0".as_ptr() as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_INVALID_TOKEN,
            b"DOCTYPE without subset not rejected\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5321 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_short_doctype_2() {
    unsafe {
        _check_set_test_info(
            b"test_short_doctype_2\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5325 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc PUBLIC></doc>\0".as_ptr() as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_SYNTAX,
            b"DOCTYPE without Public ID not rejected\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5328 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_short_doctype_3() {
    unsafe {
        _check_set_test_info(
            b"test_short_doctype_3\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5332 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM></doc>\0".as_ptr() as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_SYNTAX,
            b"DOCTYPE without System ID not rejected\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5335 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_long_doctype() {
    unsafe {
        _check_set_test_info(
            b"test_long_doctype\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5339 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc PUBLIC 'foo' 'bar' 'baz'></doc>\0".as_ptr()
                as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_SYNTAX,
            b"DOCTYPE with extra ID not rejected\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5341 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_bad_entity() {
    unsafe {
        _check_set_test_info(
            b"test_bad_entity\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5345 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [\n  <!ENTITY foo PUBLIC>\n]>\n<doc/>\0".as_ptr()
                as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_SYNTAX,
            b"ENTITY without Public ID is not rejected\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5351 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_bad_entity_2() {
    unsafe {
        _check_set_test_info(
            b"test_bad_entity_2\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5356 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [\n  <!ENTITY % foo bar>\n]>\n<doc/>\0".as_ptr()
                as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_SYNTAX,
            b"ENTITY without Public ID is not rejected\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5362 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_bad_entity_3() {
    unsafe {
        _check_set_test_info(
            b"test_bad_entity_3\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5366 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [\n  <!ENTITY % foo PUBLIC>\n]>\n<doc/>\0".as_ptr()
                as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_SYNTAX,
            b"Parameter ENTITY without Public ID is not rejected\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5372 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_bad_entity_4() {
    unsafe {
        _check_set_test_info(
            b"test_bad_entity_4\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5376 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [\n  <!ENTITY % foo SYSTEM>\n]>\n<doc/>\0".as_ptr()
                as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_SYNTAX,
            b"Parameter ENTITY without Public ID is not rejected\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5382 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_bad_notation() {
    unsafe {
        _check_set_test_info(
            b"test_bad_notation\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5386 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [\n  <!NOTATION n SYSTEM>\n]>\n<doc/>\0".as_ptr()
                as *const ::core::ffi::c_char;
        _expect_failure(
            text,
            XML_ERROR_SYNTAX,
            b"Notation without System ID is not rejected\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5392 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_default_doctype_handler() {
    unsafe {
        _check_set_test_info(
            b"test_default_doctype_handler\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5397 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc PUBLIC 'pubname' 'test.dtd' [\n  <!ENTITY foo 'bar'>\n]>\n<doc>&foo;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut test_data: [DefaultCheck; 3] = [
            default_check {
                expected: b"'pubname'\0".as_ptr() as *const XML_Char,
                expectedLen: 9 as ::core::ffi::c_int,
                seen: XML_FALSE,
            },
            default_check {
                expected: b"'test.dtd'\0".as_ptr() as *const XML_Char,
                expectedLen: 10 as ::core::ffi::c_int,
                seen: XML_FALSE,
            },
            default_check {
                expected: ::core::ptr::null::<XML_Char>(),
                expectedLen: 0 as ::core::ffi::c_int,
                seen: XML_FALSE,
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
        XML_SetDefaultHandler(
            g_parser,
            Some(
                checking_default_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetEntityDeclHandler(
            g_parser,
            Some(
                dummy_entity_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                        *const XML_Char,
                        ::core::ffi::c_int,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5412 as ::core::ffi::c_int,
            );
        }
        i = 0 as ::core::ffi::c_int;
        while !test_data[i as usize].expected.is_null() {
            if test_data[i as usize].seen == 0 {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5415 as ::core::ffi::c_int,
                    b"Default handler not run for public !DOCTYPE\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            i += 1;
        }
    }
}
extern "C" fn test_empty_element_abort() {
    unsafe {
        _check_set_test_info(
            b"test_empty_element_abort\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5419 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<abort/>\0".as_ptr() as *const ::core::ffi::c_char;
        XML_SetStartElementHandler(
            g_parser,
            Some(
                start_element_suspender
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5425 as ::core::ffi::c_int,
                b"Expected to error on abort\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_pool_integrity_with_unfinished_attr() {
    unsafe {
        _check_set_test_info(
            b"test_pool_integrity_with_unfinished_attr\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5432 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='UTF-8'?>\n<!DOCTYPE foo [\n<!ELEMENT foo ANY>\n<!ENTITY % entp SYSTEM \"external.dtd\">\n%entp;\n]>\n<a></a>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut expected: *const XML_Char = b"COMMENT\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_unfinished_attlist
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetAttlistDeclHandler(
            g_parser,
            Some(
                dummy_attlist_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetCommentHandler(
            g_parser,
            Some(
                accumulate_comment
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
            ),
        );
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5451 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
extern "C" fn test_entity_ref_no_elements() {
    unsafe {
        _check_set_test_info(
            b"test_entity_ref_no_elements\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5457 as ::core::ffi::c_int,
        );
        let text: *const ::core::ffi::c_char =
            b"<!DOCTYPE foo [\n<!ENTITY e1 \"test\">\n]> <foo>&e1;\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if !(_XML_Parse_SINGLE_BYTES(
            parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            1 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                5464 as ::core::ffi::c_int,
                b"check failed: _XML_Parse_SINGLE_BYTES(parser, text, (int)strlen(text), XML_TRUE) == XML_STATUS_ERROR\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !(XML_GetErrorCode(parser) as ::core::ffi::c_uint
            == XML_ERROR_NO_ELEMENTS as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5465 as ::core::ffi::c_int,
                b"check failed: XML_GetErrorCode(parser) == XML_ERROR_NO_ELEMENTS\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        XML_ParserFree(parser);
    }
}
extern "C" fn test_deep_nested_entity() {
    unsafe {
        _check_set_test_info(
            b"test_deep_nested_entity\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5471 as ::core::ffi::c_int,
        );
        let N_LINES: size_t = 60000 as size_t;
        let SIZE_PER_LINE: size_t = 50 as size_t;
        let text: *mut ::core::ffi::c_char = malloc(
            N_LINES
                .wrapping_add(4 as size_t)
                .wrapping_mul(SIZE_PER_LINE),
        ) as *mut ::core::ffi::c_char;
        if text.is_null() {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5477 as ::core::ffi::c_int,
                b"malloc failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut textPtr: *mut ::core::ffi::c_char = text;
        textPtr = textPtr.offset(snprintf(
            textPtr,
            SIZE_PER_LINE,
            b"<!DOCTYPE foo [\n\t<!ENTITY s0 'deepText'>\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        ) as isize);
        let mut i: size_t = 1 as size_t;
        while i < N_LINES {
            textPtr = textPtr.offset(snprintf(
                textPtr,
                SIZE_PER_LINE,
                b"  <!ENTITY s%lu '&s%lu;'>\n\0".as_ptr() as *const ::core::ffi::c_char,
                i as ::core::ffi::c_ulong,
                i.wrapping_sub(1 as size_t) as ::core::ffi::c_ulong,
            ) as isize);
            i = i.wrapping_add(1);
        }
        snprintf(
            textPtr,
            SIZE_PER_LINE,
            b"]> <foo>&s%lu;</foo>\n\0".as_ptr() as *const ::core::ffi::c_char,
            N_LINES.wrapping_sub(1 as size_t) as ::core::ffi::c_ulong,
        );
        let expected: *const XML_Char = b"deepText\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        XML_SetCharacterDataHandler(
            parser,
            Some(
                accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(parser, &raw mut storage as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5507 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
        XML_ParserFree(parser);
        free(text as *mut ::core::ffi::c_void);
    }
}
extern "C" fn test_deep_nested_attribute_entity() {
    unsafe {
        _check_set_test_info(
            b"test_deep_nested_attribute_entity\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5517 as ::core::ffi::c_int,
        );
        let N_LINES: size_t = 60000 as size_t;
        let SIZE_PER_LINE: size_t = 100 as size_t;
        let text: *mut ::core::ffi::c_char = malloc(
            N_LINES
                .wrapping_add(4 as size_t)
                .wrapping_mul(SIZE_PER_LINE),
        ) as *mut ::core::ffi::c_char;
        if text.is_null() {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5523 as ::core::ffi::c_int,
                b"malloc failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut textPtr: *mut ::core::ffi::c_char = text;
        textPtr = textPtr.offset(snprintf(
            textPtr,
            SIZE_PER_LINE,
            b"<!DOCTYPE foo [\n\t<!ENTITY s0 'deepText'>\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        ) as isize);
        let mut i: size_t = 1 as size_t;
        while i < N_LINES {
            textPtr = textPtr.offset(snprintf(
                textPtr,
                SIZE_PER_LINE,
                b"  <!ENTITY s%lu '&s%lu;'>\n\0".as_ptr() as *const ::core::ffi::c_char,
                i as ::core::ffi::c_ulong,
                i.wrapping_sub(1 as size_t) as ::core::ffi::c_ulong,
            ) as isize);
            i = i.wrapping_add(1);
        }
        snprintf(
            textPtr,
            SIZE_PER_LINE,
            b"]> <foo name='&s%lu;'>mainText</foo>\n\0".as_ptr() as *const ::core::ffi::c_char,
            N_LINES.wrapping_sub(1 as size_t) as ::core::ffi::c_ulong,
        );
        let mut doc_info: [AttrInfo; 2] = [
            attrInfo {
                name: b"name\0".as_ptr() as *const XML_Char,
                value: b"deepText\0".as_ptr() as *const XML_Char,
            },
            attrInfo {
                name: ::core::ptr::null::<XML_Char>(),
                value: ::core::ptr::null::<XML_Char>(),
            },
        ];
        let mut info: [ElementInfo; 2] = [
            elementInfo {
                name: b"foo\0".as_ptr() as *const XML_Char,
                attr_count: 1 as ::core::ffi::c_int,
                id_name: ::core::ptr::null::<XML_Char>(),
                attributes: ::core::ptr::null_mut::<AttrInfo>(),
            },
            elementInfo {
                name: ::core::ptr::null::<XML_Char>(),
                attr_count: 0 as ::core::ffi::c_int,
                id_name: ::core::ptr::null::<XML_Char>(),
                attributes: ::core::ptr::null_mut::<AttrInfo>(),
            },
        ];
        info[0 as ::core::ffi::c_int as usize].attributes = &raw mut doc_info as *mut AttrInfo;
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        let mut parserPlusElemenInfo: ParserAndElementInfo = StructParserAndElementInfo {
            parser: parser,
            info: &raw mut info as *mut ElementInfo,
        };
        XML_SetStartElementHandler(
            parser,
            Some(
                counting_start_element_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetUserData(
            parser,
            &raw mut parserPlusElemenInfo as *mut ::core::ffi::c_void,
        );
        if _XML_Parse_SINGLE_BYTES(
            parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5553 as ::core::ffi::c_int,
            );
        }
        XML_ParserFree(parser);
        free(text as *mut ::core::ffi::c_void);
    }
}
extern "C" fn test_deep_nested_entity_delayed_interpretation() {
    unsafe {
        _check_set_test_info(
            b"test_deep_nested_entity_delayed_interpretation\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5560 as ::core::ffi::c_int,
        );
        let N_LINES: size_t = 70000 as ::core::ffi::c_int as size_t;
        let SIZE_PER_LINE: size_t = 100 as size_t;
        let text: *mut ::core::ffi::c_char = malloc(
            N_LINES
                .wrapping_add(4 as size_t)
                .wrapping_mul(SIZE_PER_LINE),
        ) as *mut ::core::ffi::c_char;
        if text.is_null() {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5566 as ::core::ffi::c_int,
                b"malloc failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut textPtr: *mut ::core::ffi::c_char = text;
        textPtr = textPtr.offset(snprintf(
            textPtr,
            SIZE_PER_LINE,
            b"<!DOCTYPE foo [\n\t<!ENTITY %% s0 'deepText'>\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        ) as isize);
        let mut i: size_t = 1 as size_t;
        while i < N_LINES {
            textPtr = textPtr.offset(snprintf(
                textPtr,
                SIZE_PER_LINE,
                b"  <!ENTITY %% s%lu '&#37;s%lu;'>\n\0".as_ptr() as *const ::core::ffi::c_char,
                i as ::core::ffi::c_ulong,
                i.wrapping_sub(1 as size_t) as ::core::ffi::c_ulong,
            ) as isize);
            i = i.wrapping_add(1);
        }
        snprintf(
            textPtr,
            SIZE_PER_LINE,
            b"  <!ENTITY %% define_g \"<!ENTITY g '&#37;s%lu;'>\">\n  %%define_g;\n]>\n<foo/>\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            N_LINES.wrapping_sub(1 as size_t) as ::core::ffi::c_ulong,
        );
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        if _XML_Parse_SINGLE_BYTES(
            parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5594 as ::core::ffi::c_int,
            );
        }
        XML_ParserFree(parser);
        free(text as *mut ::core::ffi::c_void);
    }
}
extern "C" fn test_nested_entity_suspend() {
    unsafe {
        _check_set_test_info(
            b"test_nested_entity_suspend\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5601 as ::core::ffi::c_int,
        );
        let text: *const ::core::ffi::c_char = b"<!DOCTYPE a [\n  <!ENTITY e1 '<!--e1-->'>\n  <!ENTITY e2 '<!--e2 head-->&e1;<!--e2 tail-->'>\n  <!ENTITY e3 '<!--e3 head-->&e2;<!--e3 tail-->'>\n]>\n<a><!--start-->&e3;<!--end--></a>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let expected: *const XML_Char =
            b"starte3 heade2 heade1e2 taile3 tailend\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        let mut parserPlusStorage: ParserPlusStorage = ParserPlusStorage {
            parser: parser,
            storage: &raw mut storage,
        };
        XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetCommentHandler(
            parser,
            Some(
                accumulate_and_suspend_comment_handler
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
            ),
        );
        XML_SetUserData(
            parser,
            &raw mut parserPlusStorage as *mut ::core::ffi::c_void,
        );
        let mut status: XML_Status = XML_Parse(
            parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        );
        while status as ::core::ffi::c_uint
            == XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            status = XML_ResumeParser(parser);
        }
        if status as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5624 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
        XML_ParserFree(parser);
    }
}
extern "C" fn test_nested_entity_suspend_2() {
    unsafe {
        _check_set_test_info(
            b"test_nested_entity_suspend_2\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5631 as ::core::ffi::c_int,
        );
        let text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY ge1 'head1Ztail1'>\n  <!ENTITY ge2 'head2&ge1;tail2'>\n  <!ENTITY ge3 'head3&ge2;tail3'>\n]>\n<doc>&ge3;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let expected: *const XML_Char =
            b"head3head2head1Ztail1tail2tail3\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        let mut parserPlusStorage: ParserPlusStorage = ParserPlusStorage {
            parser: parser,
            storage: &raw mut storage,
        };
        XML_SetCharacterDataHandler(
            parser,
            Some(
                accumulate_char_data_and_suspend
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(
            parser,
            &raw mut parserPlusStorage as *mut ::core::ffi::c_void,
        );
        let mut status: XML_Status = XML_Parse(
            parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        );
        while status as ::core::ffi::c_uint
            == XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            status = XML_ResumeParser(parser);
        }
        if status as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5653 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
        XML_ParserFree(parser);
    }
}
extern "C" fn test_big_tokens_scale_linearly() {
    unsafe {
        _check_set_test_info(
            b"test_big_tokens_scale_linearly\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5661 as ::core::ffi::c_int,
        );
        let text: [C2Rust_Unnamed; 5] = [
            C2Rust_Unnamed {
                pre: b"<a>\0".as_ptr() as *const ::core::ffi::c_char,
                post: b"</a>\0".as_ptr() as *const ::core::ffi::c_char,
            },
            C2Rust_Unnamed {
                pre: b"<b><![CDATA[ value: \0".as_ptr() as *const ::core::ffi::c_char,
                post: b" ]]></b>\0".as_ptr() as *const ::core::ffi::c_char,
            },
            C2Rust_Unnamed {
                pre: b"<c attr='\0".as_ptr() as *const ::core::ffi::c_char,
                post: b"'></c>\0".as_ptr() as *const ::core::ffi::c_char,
            },
            C2Rust_Unnamed {
                pre: b"<d><!-- \0".as_ptr() as *const ::core::ffi::c_char,
                post: b" --></d>\0".as_ptr() as *const ::core::ffi::c_char,
            },
            C2Rust_Unnamed {
                pre: b"<e><\0".as_ptr() as *const ::core::ffi::c_char,
                post: b"/></e>\0".as_ptr() as *const ::core::ffi::c_char,
            },
        ];
        let num_cases: ::core::ffi::c_int = (::core::mem::size_of::<[C2Rust_Unnamed; 5]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed>() as usize)
            as ::core::ffi::c_int;
        let mut aaaaaa: [::core::ffi::c_char; 4096] = [0; 4096];
        let fillsize: ::core::ffi::c_int =
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as ::core::ffi::c_int;
        let fillcount: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
        let approx_bytes: ::core::ffi::c_uint = (fillsize * fillcount) as ::core::ffi::c_uint;
        let max_factor: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
        let max_scanned: ::core::ffi::c_uint = max_factor.wrapping_mul(approx_bytes);
        memset(
            &raw mut aaaaaa as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            'a' as i32,
            fillsize as size_t,
        );
        if g_reparseDeferralEnabledDefault == 0 {
            return;
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < num_cases {
            let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
            if parser.is_null() {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5688 as ::core::ffi::c_int,
                    b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            let mut status: XML_Status = XML_STATUS_ERROR;
            set_subtest(
                b"text=\"%saaaaaa%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                text[i as usize].pre,
                text[i as usize].post,
            );
            g_bytesScanned = 0 as ::core::ffi::c_uint;
            status = _XML_Parse_SINGLE_BYTES(
                parser,
                text[i as usize].pre,
                strlen(text[i as usize].pre) as ::core::ffi::c_int,
                XML_FALSE as ::core::ffi::c_int,
            );
            if status as ::core::ffi::c_uint
                != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _xml_failure(
                    parser,
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5697 as ::core::ffi::c_int,
                );
            }
            let mut past_max_count: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            let mut f: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while f < fillcount {
                status = _XML_Parse_SINGLE_BYTES(
                    parser,
                    &raw mut aaaaaa as *mut ::core::ffi::c_char,
                    fillsize,
                    XML_FALSE as ::core::ffi::c_int,
                );
                if status as ::core::ffi::c_uint
                    != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    _xml_failure(
                        parser,
                        b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5705 as ::core::ffi::c_int,
                    );
                }
                if g_bytesScanned > max_scanned {
                    let pushed: ::core::ffi::c_uint =
                        (strlen(text[i as usize].pre) as ::core::ffi::c_uint).wrapping_add(
                            ((f + 1 as ::core::ffi::c_int) * fillsize) as ::core::ffi::c_uint,
                        );
                    fprintf(
                        stderr,
                        b"after %d/%d loops: pushed=%u scanned=%u (factor ~%.2f) max_scanned: %u (factor ~%u)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        f + 1 as ::core::ffi::c_int,
                        fillcount,
                        pushed,
                        g_bytesScanned,
                        g_bytesScanned as ::core::ffi::c_double
                            / pushed as ::core::ffi::c_double,
                        max_scanned,
                        max_factor,
                    );
                    past_max_count = past_max_count.wrapping_add(1);
                    if !(past_max_count < 5 as ::core::ffi::c_uint) {
                        _fail(
                            b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            5720 as ::core::ffi::c_int,
                            b"check failed: past_max_count < 5\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                }
                f += 1;
            }
            status = _XML_Parse_SINGLE_BYTES(
                parser,
                text[i as usize].post,
                strlen(text[i as usize].post) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            );
            if status as ::core::ffi::c_uint
                != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _xml_failure(
                    parser,
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5728 as ::core::ffi::c_int,
                );
            }
            if !(g_bytesScanned > approx_bytes) {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5731 as ::core::ffi::c_int,
                    b"check failed: g_bytesScanned > approx_bytes\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            if g_bytesScanned > max_scanned {
                fprintf(
                    stderr,
                    b"after all input: scanned=%u (factor ~%.2f) max_scanned: %u (factor ~%u)\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    g_bytesScanned,
                    g_bytesScanned as ::core::ffi::c_double / approx_bytes as ::core::ffi::c_double,
                    max_scanned,
                    max_factor,
                );
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5738 as ::core::ffi::c_int,
                    b"scanned too many bytes\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            XML_ParserFree(parser);
            i += 1;
        }
    }
}
extern "C" fn test_set_reparse_deferral() {
    unsafe {
        _check_set_test_info(
            b"test_set_reparse_deferral\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5746 as ::core::ffi::c_int,
        );
        let pre: *const ::core::ffi::c_char = b"<d>\0".as_ptr() as *const ::core::ffi::c_char;
        let start: *const ::core::ffi::c_char =
            b"<x attr='\0".as_ptr() as *const ::core::ffi::c_char;
        let end: *const ::core::ffi::c_char = b"'></x>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut eeeeee: [::core::ffi::c_char; 100] = [0; 100];
        let fillsize: ::core::ffi::c_int =
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int;
        memset(
            &raw mut eeeeee as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            'e' as i32,
            fillsize as size_t,
        );
        let mut enabled: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while enabled <= 1 as ::core::ffi::c_int {
            set_subtest(
                b"deferral=%d\0".as_ptr() as *const ::core::ffi::c_char,
                enabled,
            );
            let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
            if parser.is_null() {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5758 as ::core::ffi::c_int,
                    b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if XML_SetReparseDeferralEnabled(parser, enabled as XML_Bool) == 0 {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5759 as ::core::ffi::c_int,
                    b"check failed: XML_SetReparseDeferralEnabled(parser, enabled)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            if XML_GetBuffer(parser, fillsize * 10103 as ::core::ffi::c_int).is_null() {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5761 as ::core::ffi::c_int,
                    b"check failed: XML_GetBuffer(parser, fillsize * 10103) != NULL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            let mut storage: CharData = CharData {
                count: 0,
                data: [0; 2048],
            };
            CharData_Init(&raw mut storage);
            XML_SetUserData(parser, &raw mut storage as *mut ::core::ffi::c_void);
            XML_SetStartElementHandler(
                parser,
                Some(
                    start_element_event_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *mut *const XML_Char,
                        ) -> (),
                ),
            );
            let mut status: XML_Status = XML_STATUS_ERROR;
            status = XML_Parse(
                parser,
                pre,
                strlen(pre) as ::core::ffi::c_int,
                XML_FALSE as ::core::ffi::c_int,
            );
            if status as ::core::ffi::c_uint
                != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _xml_failure(
                    parser,
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5772 as ::core::ffi::c_int,
                );
            }
            CharData_CheckXMLChars(&raw mut storage, b"d\0".as_ptr() as *const XML_Char);
            status = XML_Parse(
                parser,
                start,
                strlen(start) as ::core::ffi::c_int,
                XML_FALSE as ::core::ffi::c_int,
            );
            if status as ::core::ffi::c_uint
                != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _xml_failure(
                    parser,
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5779 as ::core::ffi::c_int,
                );
            }
            CharData_CheckXMLChars(&raw mut storage, b"d\0".as_ptr() as *const XML_Char);
            let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while c < 100 as ::core::ffi::c_int {
                status = XML_Parse(
                    parser,
                    &raw mut eeeeee as *mut ::core::ffi::c_char,
                    fillsize,
                    XML_FALSE as ::core::ffi::c_int,
                );
                if status as ::core::ffi::c_uint
                    != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    _xml_failure(
                        parser,
                        b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5787 as ::core::ffi::c_int,
                    );
                }
                c += 1;
            }
            CharData_CheckXMLChars(&raw mut storage, b"d\0".as_ptr() as *const XML_Char);
            status = XML_Parse(
                parser,
                end,
                strlen(end) as ::core::ffi::c_int,
                XML_FALSE as ::core::ffi::c_int,
            );
            if status as ::core::ffi::c_uint
                != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _xml_failure(
                    parser,
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5795 as ::core::ffi::c_int,
                );
            }
            if enabled != 0 {
                CharData_CheckXMLChars(&raw mut storage, b"d\0".as_ptr() as *const XML_Char);
                let mut c_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while c_0 < 101 as ::core::ffi::c_int {
                    status = XML_Parse(
                        parser,
                        &raw mut eeeeee as *mut ::core::ffi::c_char,
                        fillsize,
                        XML_FALSE as ::core::ffi::c_int,
                    );
                    if status as ::core::ffi::c_uint
                        != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        _xml_failure(
                            parser,
                            b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            5806 as ::core::ffi::c_int,
                        );
                    }
                    c_0 += 1;
                }
            }
            CharData_CheckXMLChars(&raw mut storage, b"dx\0".as_ptr() as *const XML_Char);
            XML_ParserFree(parser);
            enabled += 1 as ::core::ffi::c_int;
        }
    }
}
extern "C" fn element_decl_counter(
    user_data: *mut ::core::ffi::c_void,
    _name: *const XML_Char,
    model: *mut XML_Content,
) {
    let testdata = mut_from_ptr(user_data as *mut element_decl_data);
    testdata.count += 1 as ::core::ffi::c_int;
    ffi_call2(XML_FreeContentModel, testdata.parser, model);
}
extern "C" fn external_inherited_parser(
    p: XML_Parser,
    context: *const XML_Char,
    _base: *const XML_Char,
    _system_id: *const XML_Char,
    _public_id: *const XML_Char,
) -> ::core::ffi::c_int {
    let pre = bytes_as_c_char_ptr(b"<!ELEMENT document ANY>\n\0");
    let start = bytes_as_c_char_ptr(b"<!ELEMENT \0");
    let end = bytes_as_c_char_ptr(b" ANY>\n\0");
    let post = bytes_as_c_char_ptr(b"<!ELEMENT xyz ANY>\n\0");
    let enabled = value_from_ptr(parser_user_data_as::<::core::ffi::c_int>(p));
    let mut eeeeee: [::core::ffi::c_char; 100] = [0; 100];
    let mut spaces: [::core::ffi::c_char; 100] = [0; 100];
    let fillsize = ::core::mem::size_of_val(&spaces) as ::core::ffi::c_int;

    eeeeee.fill(b'e' as ::core::ffi::c_char);
    spaces.fill(b' ' as ::core::ffi::c_char);

    let parser = ffi_call3(
        XML_ExternalEntityParserCreate,
        p,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if parser.is_null() {
        fail_test(
            5850 as ::core::ffi::c_int,
            b"check failed: parser != NULL\0",
        );
    }
    if parser_buffer_for(parser, fillsize * 10103 as ::core::ffi::c_int).is_null() {
        fail_test(
            5852 as ::core::ffi::c_int,
            b"check failed: XML_GetBuffer(parser, fillsize * 10103) != NULL\0",
        );
    }

    let mut testdata = element_decl_data { parser, count: 0 };
    ffi_call2(
        XML_SetUserData,
        parser,
        (&mut testdata as *mut element_decl_data).cast(),
    );
    ffi_call2(
        XML_SetElementDeclHandler,
        parser,
        Some(
            element_decl_counter
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Content,
                ) -> (),
        ),
    );

    ensure_parser_success_for(
        parser,
        parser_parse_for(
            parser,
            pre,
            c_string_len(pre),
            XML_FALSE as ::core::ffi::c_int,
        ),
        5864 as ::core::ffi::c_int,
    );
    if testdata.count != 1 as ::core::ffi::c_int {
        fail_test(
            5866 as ::core::ffi::c_int,
            b"check failed: testdata.count == 1\0",
        );
    }

    ensure_parser_success_for(
        parser,
        parser_parse_for(
            parser,
            start,
            c_string_len(start),
            XML_FALSE as ::core::ffi::c_int,
        ),
        5871 as ::core::ffi::c_int,
    );
    if testdata.count != 1 as ::core::ffi::c_int {
        fail_test(
            5873 as ::core::ffi::c_int,
            b"check failed: testdata.count == 1\0",
        );
    }

    let mut c = 0 as ::core::ffi::c_int;
    while c < 100 as ::core::ffi::c_int {
        ensure_parser_success_for(
            parser,
            parser_parse_for(
                parser,
                eeeeee.as_mut_ptr(),
                fillsize,
                XML_FALSE as ::core::ffi::c_int,
            ),
            5879 as ::core::ffi::c_int,
        );
        c += 1;
    }
    if testdata.count != 1 as ::core::ffi::c_int {
        fail_test(
            5882 as ::core::ffi::c_int,
            b"check failed: testdata.count == 1\0",
        );
    }

    ensure_parser_success_for(
        parser,
        parser_parse_for(
            parser,
            end,
            c_string_len(end),
            XML_FALSE as ::core::ffi::c_int,
        ),
        5887 as ::core::ffi::c_int,
    );
    if enabled != 0 {
        if testdata.count != 1 as ::core::ffi::c_int {
            fail_test(
                5893 as ::core::ffi::c_int,
                b"check failed: testdata.count == 1\0",
            );
        }
        let mut c_0 = 0 as ::core::ffi::c_int;
        while c_0 < 101 as ::core::ffi::c_int {
            ensure_parser_success_for(
                parser,
                parser_parse_for(
                    parser,
                    spaces.as_mut_ptr(),
                    fillsize,
                    XML_FALSE as ::core::ffi::c_int,
                ),
                5898 as ::core::ffi::c_int,
            );
            c_0 += 1;
        }
    }
    if testdata.count != 2 as ::core::ffi::c_int {
        fail_test(
            5902 as ::core::ffi::c_int,
            b"check failed: testdata.count == 2\0",
        );
    }

    ensure_parser_success_for(
        parser,
        parser_parse_for(
            parser,
            post,
            c_string_len(post),
            XML_TRUE as ::core::ffi::c_int,
        ),
        5907 as ::core::ffi::c_int,
    );
    if testdata.count != 3 as ::core::ffi::c_int {
        fail_test(
            5909 as ::core::ffi::c_int,
            b"check failed: testdata.count == 3\0",
        );
    }

    parser_free(parser);
    XML_STATUS_OK as ::core::ffi::c_int
}
extern "C" fn test_reparse_deferral_is_inherited() {
    unsafe {
        _check_set_test_info(
            b"test_reparse_deferral_is_inherited\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5915 as ::core::ffi::c_int,
        );
        let text: *const ::core::ffi::c_char =
            b"<!DOCTYPE document SYSTEM 'something.ext'><document/>\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut enabled: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while enabled <= 1 as ::core::ffi::c_int {
            set_subtest(
                b"deferral=%d\0".as_ptr() as *const ::core::ffi::c_char,
                enabled,
            );
            let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
            if parser.is_null() {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5922 as ::core::ffi::c_int,
                    b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            XML_SetUserData(parser, &raw mut enabled as *mut ::core::ffi::c_void);
            XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                parser,
                Some(
                    external_inherited_parser
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                ),
            );
            if XML_SetReparseDeferralEnabled(parser, enabled as XML_Bool) == 0 {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5928 as ::core::ffi::c_int,
                    b"check failed: XML_SetReparseDeferralEnabled(parser, enabled)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            if XML_Parse(
                parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _xml_failure(
                    parser,
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5930 as ::core::ffi::c_int,
                );
            }
            XML_ParserFree(parser);
            enabled += 1;
        }
    }
}
extern "C" fn test_set_reparse_deferral_on_null_parser() {
    unsafe {
        _check_set_test_info(
            b"test_set_reparse_deferral_on_null_parser\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5937 as ::core::ffi::c_int,
        );
        if !(XML_SetReparseDeferralEnabled(
            ::core::ptr::null_mut::<XML_ParserStruct>(),
            0 as XML_Bool,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5938 as ::core::ffi::c_int,
                b"check failed: XML_SetReparseDeferralEnabled(NULL, 0) == XML_FALSE\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !(XML_SetReparseDeferralEnabled(
            ::core::ptr::null_mut::<XML_ParserStruct>(),
            1 as XML_Bool,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5939 as ::core::ffi::c_int,
                b"check failed: XML_SetReparseDeferralEnabled(NULL, 1) == XML_FALSE\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !(XML_SetReparseDeferralEnabled(
            ::core::ptr::null_mut::<XML_ParserStruct>(),
            10 as XML_Bool,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5940 as ::core::ffi::c_int,
                b"check failed: XML_SetReparseDeferralEnabled(NULL, 10) == XML_FALSE\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !(XML_SetReparseDeferralEnabled(
            ::core::ptr::null_mut::<XML_ParserStruct>(),
            100 as XML_Bool,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5941 as ::core::ffi::c_int,
                b"check failed: XML_SetReparseDeferralEnabled(NULL, 100) == XML_FALSE\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !(XML_SetReparseDeferralEnabled(
            ::core::ptr::null_mut::<XML_ParserStruct>(),
            (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as XML_Bool,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                5943 as ::core::ffi::c_int,
                b"check failed: XML_SetReparseDeferralEnabled(NULL, (XML_Bool)INT_MIN) == XML_FALSE\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !(XML_SetReparseDeferralEnabled(
            ::core::ptr::null_mut::<XML_ParserStruct>(),
            2147483647 as ::core::ffi::c_int as XML_Bool,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                5945 as ::core::ffi::c_int,
                b"check failed: XML_SetReparseDeferralEnabled(NULL, (XML_Bool)INT_MAX) == XML_FALSE\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_set_reparse_deferral_on_the_fly() {
    unsafe {
        _check_set_test_info(
            b"test_set_reparse_deferral_on_the_fly\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            5949 as ::core::ffi::c_int,
        );
        let pre: *const ::core::ffi::c_char =
            b"<d><x attr='\0".as_ptr() as *const ::core::ffi::c_char;
        let end: *const ::core::ffi::c_char = b"'></x>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut iiiiii: [::core::ffi::c_char; 100] = [0; 100];
        let fillsize: ::core::ffi::c_int =
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int;
        memset(
            &raw mut iiiiii as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            'i' as i32,
            fillsize as size_t,
        );
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if parser.is_null() {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5957 as ::core::ffi::c_int,
                b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_SetReparseDeferralEnabled(parser, 1 as ::core::ffi::c_int as XML_Bool) == 0 {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5958 as ::core::ffi::c_int,
                b"check failed: XML_SetReparseDeferralEnabled(parser, XML_TRUE)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetUserData(parser, &raw mut storage as *mut ::core::ffi::c_void);
        XML_SetStartElementHandler(
            parser,
            Some(
                start_element_event_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        let mut status: XML_Status = XML_STATUS_ERROR;
        status = XML_Parse(
            parser,
            pre,
            strlen(pre) as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        );
        if status as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5969 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, b"d\0".as_ptr() as *const XML_Char);
        status = XML_Parse(
            parser,
            &raw mut iiiiii as *mut ::core::ffi::c_char,
            fillsize,
            XML_FALSE as ::core::ffi::c_int,
        );
        if status as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5976 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, b"d\0".as_ptr() as *const XML_Char);
        status = XML_Parse(
            parser,
            end,
            strlen(end) as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        );
        if status as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5983 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, b"d\0".as_ptr() as *const XML_Char);
        if XML_SetReparseDeferralEnabled(parser, 0 as ::core::ffi::c_int as XML_Bool) == 0 {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5988 as ::core::ffi::c_int,
                b"check failed: XML_SetReparseDeferralEnabled(parser, XML_FALSE)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        status = XML_Parse(
            parser,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        );
        if status as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                parser,
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                5992 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, b"dx\0".as_ptr() as *const XML_Char);
        XML_ParserFree(parser);
    }
}
extern "C" fn test_set_bad_reparse_option() {
    unsafe {
        _check_set_test_info(
            b"test_set_bad_reparse_option\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            6000 as ::core::ffi::c_int,
        );
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if !(0 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int
            == XML_SetReparseDeferralEnabled(parser, 2 as XML_Bool) as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                6002 as ::core::ffi::c_int,
                b"check failed: XML_FALSE == XML_SetReparseDeferralEnabled(parser, 2)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !(0 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int
            == XML_SetReparseDeferralEnabled(parser, 3 as XML_Bool) as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                6003 as ::core::ffi::c_int,
                b"check failed: XML_FALSE == XML_SetReparseDeferralEnabled(parser, 3)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !(0 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int
            == XML_SetReparseDeferralEnabled(parser, 99 as XML_Bool) as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                6004 as ::core::ffi::c_int,
                b"check failed: XML_FALSE == XML_SetReparseDeferralEnabled(parser, 99)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !(0 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int
            == XML_SetReparseDeferralEnabled(parser, 127 as XML_Bool) as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                6005 as ::core::ffi::c_int,
                b"check failed: XML_FALSE == XML_SetReparseDeferralEnabled(parser, 127)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !(0 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int
            == XML_SetReparseDeferralEnabled(parser, 128 as XML_Bool) as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                6006 as ::core::ffi::c_int,
                b"check failed: XML_FALSE == XML_SetReparseDeferralEnabled(parser, 128)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !(0 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int
            == XML_SetReparseDeferralEnabled(parser, 129 as XML_Bool) as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                6007 as ::core::ffi::c_int,
                b"check failed: XML_FALSE == XML_SetReparseDeferralEnabled(parser, 129)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !(0 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int
            == XML_SetReparseDeferralEnabled(parser, 255 as XML_Bool) as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                6008 as ::core::ffi::c_int,
                b"check failed: XML_FALSE == XML_SetReparseDeferralEnabled(parser, 255)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !(1 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int
            == XML_SetReparseDeferralEnabled(parser, 0 as XML_Bool) as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                6009 as ::core::ffi::c_int,
                b"check failed: XML_TRUE == XML_SetReparseDeferralEnabled(parser, 0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !(1 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int
            == XML_SetReparseDeferralEnabled(parser, 1 as XML_Bool) as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                6010 as ::core::ffi::c_int,
                b"check failed: XML_TRUE == XML_SetReparseDeferralEnabled(parser, 1)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        XML_ParserFree(parser);
    }
}
static G_TOTAL_ALLOC: AtomicUsize = AtomicUsize::new(0);
static G_BIGGEST_ALLOC: AtomicUsize = AtomicUsize::new(0);

fn total_alloc() -> size_t {
    G_TOTAL_ALLOC.load(Ordering::Relaxed) as size_t
}

fn biggest_alloc() -> size_t {
    G_BIGGEST_ALLOC.load(Ordering::Relaxed) as size_t
}

fn reset_alloc_counters() {
    G_TOTAL_ALLOC.store(0, Ordering::Relaxed);
    G_BIGGEST_ALLOC.store(0, Ordering::Relaxed);
}

fn note_allocation(size: size_t) {
    G_TOTAL_ALLOC.fetch_add(size as usize, Ordering::Relaxed);
    G_BIGGEST_ALLOC.fetch_max(size as usize, Ordering::Relaxed);
}

extern "C" fn counting_realloc(
    ptr: *mut ::core::ffi::c_void,
    size: size_t,
) -> *mut ::core::ffi::c_void {
    note_allocation(size);
    ffi_call2(realloc, ptr, size)
}

extern "C" fn counting_malloc(size: size_t) -> *mut ::core::ffi::c_void {
    counting_realloc(NULL, size)
}
extern "C" fn test_bypass_heuristic_when_close_to_bufsize() {
    unsafe {
        _check_set_test_info(
            b"test_bypass_heuristic_when_close_to_bufsize\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            6032 as ::core::ffi::c_int,
        );
        if g_chunkSize != 0 as ::core::ffi::c_int {
            return;
        }
        if g_reparseDeferralEnabledDefault == 0 {
            return;
        }
        let document_length: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
        let document: *mut ::core::ffi::c_char =
            malloc(document_length as size_t) as *mut ::core::ffi::c_char;
        let memfuncs: XML_Memory_Handling_Suite = XML_Memory_Handling_Suite {
            malloc_fcn: Some(
                counting_malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void,
            ),
            realloc_fcn: Some(
                counting_realloc
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                    ) -> *mut ::core::ffi::c_void,
            ),
            free_fcn: Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        };
        let leading_list: [::core::ffi::c_int; 10] = [
            0 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
            61 as ::core::ffi::c_int,
            96 as ::core::ffi::c_int,
            400 as ::core::ffi::c_int,
            401 as ::core::ffi::c_int,
            4000 as ::core::ffi::c_int,
            4010 as ::core::ffi::c_int,
            4099 as ::core::ffi::c_int,
            -(1 as ::core::ffi::c_int),
        ];
        let bigtoken_list: [::core::ffi::c_int; 8] = [
            3000 as ::core::ffi::c_int,
            4000 as ::core::ffi::c_int,
            4001 as ::core::ffi::c_int,
            4096 as ::core::ffi::c_int,
            4099 as ::core::ffi::c_int,
            5000 as ::core::ffi::c_int,
            20000 as ::core::ffi::c_int,
            -(1 as ::core::ffi::c_int),
        ];
        let fillsize_list: [::core::ffi::c_int; 9] = [
            131 as ::core::ffi::c_int,
            256 as ::core::ffi::c_int,
            399 as ::core::ffi::c_int,
            400 as ::core::ffi::c_int,
            401 as ::core::ffi::c_int,
            1025 as ::core::ffi::c_int,
            4099 as ::core::ffi::c_int,
            4321 as ::core::ffi::c_int,
            -(1 as ::core::ffi::c_int),
        ];
        let mut leading: *const ::core::ffi::c_int =
            &raw const leading_list as *const ::core::ffi::c_int;
        while *leading >= 0 as ::core::ffi::c_int {
            let mut bigtoken: *const ::core::ffi::c_int =
                &raw const bigtoken_list as *const ::core::ffi::c_int;
            while *bigtoken >= 0 as ::core::ffi::c_int {
                let mut fillsize: *const ::core::ffi::c_int =
                    &raw const fillsize_list as *const ::core::ffi::c_int;
                while *fillsize >= 0 as ::core::ffi::c_int {
                    set_subtest(
                        b"leading=%d bigtoken=%d fillsize=%d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        *leading,
                        *bigtoken,
                        *fillsize,
                    );
                    if !(*leading + *bigtoken <= document_length) {
                        _fail(
                            b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            6061 as ::core::ffi::c_int,
                            b"check failed: *leading + *bigtoken <= document_length\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                    memset(
                        document as *mut ::core::ffi::c_void,
                        'x' as i32,
                        document_length as size_t,
                    );
                    if *leading != 0 {
                        if !(*leading >= 3 as ::core::ffi::c_int) {
                            _fail(
                                b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                6067 as ::core::ffi::c_int,
                                b"check failed: *leading >= 3\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        memcpy(
                            document as *mut ::core::ffi::c_void,
                            b"<a>\0".as_ptr() as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            3 as size_t,
                        );
                    }
                    *document.offset((*leading + 0 as ::core::ffi::c_int) as isize) =
                        '<' as i32 as ::core::ffi::c_char;
                    *document.offset((*leading + 1 as ::core::ffi::c_int) as isize) =
                        'b' as i32 as ::core::ffi::c_char;
                    memset(
                        document.offset((*leading + 2 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        ' ' as i32,
                        (*bigtoken - 2 as ::core::ffi::c_int) as size_t,
                    );
                    *document.offset((*leading + *bigtoken - 1 as ::core::ffi::c_int) as isize) =
                        '>' as i32 as ::core::ffi::c_char;
                    let expected_elem_total: ::core::ffi::c_int = 1 as ::core::ffi::c_int
                        + (if *leading != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        });
                    let mut parser: XML_Parser = XML_ParserCreate_MM(
                        ::core::ptr::null::<XML_Char>(),
                        &raw const memfuncs,
                        ::core::ptr::null::<XML_Char>(),
                    );
                    if parser.is_null() {
                        _fail(
                            b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            6080 as ::core::ffi::c_int,
                            b"check failed: parser != NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                    let mut storage: CharData = CharData {
                        count: 0,
                        data: [0; 2048],
                    };
                    CharData_Init(&raw mut storage);
                    XML_SetUserData(parser, &raw mut storage as *mut ::core::ffi::c_void);
                    XML_SetStartElementHandler(
                        parser,
                        Some(
                            start_element_event_handler
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *const XML_Char,
                                    *mut *const XML_Char,
                                ) -> (),
                        ),
                    );
                    reset_alloc_counters();
                    let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while offset < *leading + *bigtoken {
                        if !(offset + *fillsize <= document_length) {
                            _fail(
                                b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                6092 as ::core::ffi::c_int,
                                b"check failed: offset + *fillsize <= document_length\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        let status: XML_Status = XML_Parse(
                            parser,
                            document.offset(offset as isize) as *mut ::core::ffi::c_char,
                            *fillsize,
                            XML_FALSE as ::core::ffi::c_int,
                        ) as XML_Status;
                        if status as ::core::ffi::c_uint
                            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            _xml_failure(
                                parser,
                                b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                6096 as ::core::ffi::c_int,
                            );
                        }
                        offset += *fillsize;
                    }
                    let bigtok_first_chunk_bytes: ::core::ffi::c_int =
                        *fillsize - *leading % *fillsize;
                    if !(bigtok_first_chunk_bytes >= *bigtoken
                        && XML_CONTEXT_BYTES == 0 as ::core::ffi::c_int)
                    {
                        if *leading < XML_CONTEXT_BYTES {
                            if !(biggest_alloc()
                                >= (*leading as size_t).wrapping_add(*bigtoken as size_t))
                            {
                                _fail(
                                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    6110 as ::core::ffi::c_int,
                                    b"check failed: g_biggestAlloc >= *leading + (size_t)*bigtoken\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                );
                            }
                        } else if !(biggest_alloc()
                            >= (1024 as size_t).wrapping_add(*bigtoken as size_t))
                        {
                            _fail(
                                b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                6112 as ::core::ffi::c_int,
                                b"check failed: g_biggestAlloc >= XML_CONTEXT_BYTES + (size_t)*bigtoken\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    while storage.count < expected_elem_total {
                        let alloc_before: size_t = total_alloc();
                        if !(offset + *fillsize <= document_length) {
                            _fail(
                                b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                6117 as ::core::ffi::c_int,
                                b"check failed: offset + *fillsize <= document_length\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        let status_0: XML_Status = XML_Parse(
                            parser,
                            document.offset(offset as isize) as *mut ::core::ffi::c_char,
                            *fillsize,
                            XML_FALSE as ::core::ffi::c_int,
                        ) as XML_Status;
                        if status_0 as ::core::ffi::c_uint
                            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            _xml_failure(
                                parser,
                                b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                6121 as ::core::ffi::c_int,
                            );
                        }
                        offset += *fillsize;
                        if !(total_alloc().wrapping_sub(alloc_before) < 4096 as size_t) {
                            _fail(
                                b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                6128 as ::core::ffi::c_int,
                                b"check failed: g_totalAlloc - alloc_before < 4096\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    if !(total_alloc() > 0 as size_t) {
                        _fail(
                            b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            6131 as ::core::ffi::c_int,
                            b"check failed: g_totalAlloc > 0\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                    if !(storage.count == expected_elem_total) {
                        _fail(
                            b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            6133 as ::core::ffi::c_int,
                            b"check failed: storage.count == expected_elem_total\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                    XML_ParserFree(parser);
                    fillsize = fillsize.offset(1);
                }
                bigtoken = bigtoken.offset(1);
            }
            leading = leading.offset(1);
        }
        free(document as *mut ::core::ffi::c_void);
    }
}
extern "C" fn test_varying_buffer_fills() {
    unsafe {
        _check_set_test_info(
            b"test_varying_buffer_fills\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            6143 as ::core::ffi::c_int,
        );
        let KiB: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
        let MiB: ::core::ffi::c_int = 1024 as ::core::ffi::c_int * KiB;
        let document_length: ::core::ffi::c_int = 16 as ::core::ffi::c_int * MiB;
        let big: ::core::ffi::c_int = 7654321 as ::core::ffi::c_int;
        if g_chunkSize != 0 as ::core::ffi::c_int {
            return;
        }
        let document: *mut ::core::ffi::c_char =
            malloc(document_length as size_t) as *mut ::core::ffi::c_char;
        if document.is_null() {
            _fail(
                b"/root/work/expat/tests/basic_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                6154 as ::core::ffi::c_int,
                b"check failed: document != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        memset(
            document as *mut ::core::ffi::c_void,
            'x' as i32,
            document_length as size_t,
        );
        *document.offset(0 as ::core::ffi::c_int as isize) = '<' as i32 as ::core::ffi::c_char;
        *document.offset(1 as ::core::ffi::c_int as isize) = 't' as i32 as ::core::ffi::c_char;
        memset(
            document.offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            ' ' as i32,
            (big - 2 as ::core::ffi::c_int) as size_t,
        );
        *document.offset((big - 1 as ::core::ffi::c_int) as isize) =
            '>' as i32 as ::core::ffi::c_char;
        let testcases: [[::core::ffi::c_int; 30]; 11] = [
            [
                8 as ::core::ffi::c_int * MiB,
                -(8 as ::core::ffi::c_int) * MiB,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                4 as ::core::ffi::c_int * MiB,
                4 as ::core::ffi::c_int * MiB,
                -(12 as ::core::ffi::c_int) * MiB,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                4 as ::core::ffi::c_int * MiB,
                0 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int * MiB,
                -(12 as ::core::ffi::c_int) * MiB,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                4 as ::core::ffi::c_int * MiB,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int * MiB,
                -(12 as ::core::ffi::c_int) * MiB,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                4 as ::core::ffi::c_int * MiB,
                0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int * MiB,
                0 as ::core::ffi::c_int,
                3 as ::core::ffi::c_int * MiB,
                -(12 as ::core::ffi::c_int) * MiB,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                4 as ::core::ffi::c_int * MiB,
                2 as ::core::ffi::c_int * MiB,
                1 as ::core::ffi::c_int * MiB,
                512 as ::core::ffi::c_int * KiB,
                256 as ::core::ffi::c_int * KiB,
                256 as ::core::ffi::c_int * KiB,
                -(12 as ::core::ffi::c_int) * MiB,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                4 as ::core::ffi::c_int * MiB + 1 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int * MiB,
                1 as ::core::ffi::c_int * MiB,
                512 as ::core::ffi::c_int * KiB,
                -(25 as ::core::ffi::c_int) * MiB,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                1 as ::core::ffi::c_int * KiB,
                2 as ::core::ffi::c_int * KiB,
                4 as ::core::ffi::c_int * KiB,
                8 as ::core::ffi::c_int * KiB,
                16 as ::core::ffi::c_int * KiB,
                32 as ::core::ffi::c_int * KiB,
                64 as ::core::ffi::c_int * KiB,
                128 as ::core::ffi::c_int * KiB,
                256 as ::core::ffi::c_int * KiB,
                512 as ::core::ffi::c_int * KiB,
                1 as ::core::ffi::c_int * MiB,
                2 as ::core::ffi::c_int * MiB,
                4 as ::core::ffi::c_int * MiB,
                -(16 as ::core::ffi::c_int) * MiB,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                2 as ::core::ffi::c_int * KiB + 1 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int * KiB,
                4 as ::core::ffi::c_int * KiB,
                8 as ::core::ffi::c_int * KiB,
                16 as ::core::ffi::c_int * KiB,
                32 as ::core::ffi::c_int * KiB,
                64 as ::core::ffi::c_int * KiB,
                128 as ::core::ffi::c_int * KiB,
                256 as ::core::ffi::c_int * KiB,
                512 as ::core::ffi::c_int * KiB,
                1 as ::core::ffi::c_int * MiB,
                2 as ::core::ffi::c_int * MiB,
                4 as ::core::ffi::c_int * MiB,
                -(10 as ::core::ffi::c_int * MiB
                    + 682 as ::core::ffi::c_int * KiB
                    + 7 as ::core::ffi::c_int),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                2 as ::core::ffi::c_int * KiB + 1 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int * KiB,
                4 as ::core::ffi::c_int * KiB,
                8 as ::core::ffi::c_int * KiB,
                16 as ::core::ffi::c_int * KiB,
                32 as ::core::ffi::c_int * KiB,
                64 as ::core::ffi::c_int * KiB,
                128 as ::core::ffi::c_int * KiB,
                256 as ::core::ffi::c_int * KiB,
                512 as ::core::ffi::c_int * KiB,
                1 as ::core::ffi::c_int * MiB,
                2 as ::core::ffi::c_int * MiB,
                4 as ::core::ffi::c_int * MiB - 1 as ::core::ffi::c_int,
                -(10 as ::core::ffi::c_int * MiB
                    + 682 as ::core::ffi::c_int * KiB
                    + 6 as ::core::ffi::c_int),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                512 as ::core::ffi::c_int * KiB + 1 as ::core::ffi::c_int,
                256 as ::core::ffi::c_int * KiB,
                128 as ::core::ffi::c_int * KiB,
                128 as ::core::ffi::c_int * KiB - 1 as ::core::ffi::c_int,
                512 as ::core::ffi::c_int * KiB + 1 as ::core::ffi::c_int,
                256 as ::core::ffi::c_int * KiB,
                128 as ::core::ffi::c_int * KiB,
                128 as ::core::ffi::c_int * KiB - 1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int * MiB + 1 as ::core::ffi::c_int,
                512 as ::core::ffi::c_int * KiB,
                256 as ::core::ffi::c_int * KiB,
                256 as ::core::ffi::c_int * KiB - 1 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int * MiB + 1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int * MiB,
                512 as ::core::ffi::c_int * KiB,
                -(45 as ::core::ffi::c_int * MiB + 12 as ::core::ffi::c_int),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        ];
        let testcount: ::core::ffi::c_int =
            (::core::mem::size_of::<[[::core::ffi::c_int; 30]; 11]>() as usize)
                .wrapping_div(::core::mem::size_of::<[::core::ffi::c_int; 30]>() as usize)
                as ::core::ffi::c_int;
        let mut test_i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while test_i < testcount {
            let mut fillsize: *const ::core::ffi::c_int =
                &raw const *(&raw const testcases as *const [::core::ffi::c_int; 30])
                    .offset(test_i as isize) as *const ::core::ffi::c_int;
            set_subtest(
                b"#%d {%d %d %d %d ...}\0".as_ptr() as *const ::core::ffi::c_char,
                test_i,
                *fillsize.offset(0 as ::core::ffi::c_int as isize),
                *fillsize.offset(1 as ::core::ffi::c_int as isize),
                *fillsize.offset(2 as ::core::ffi::c_int as isize),
                *fillsize.offset(3 as ::core::ffi::c_int as isize),
            );
            let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
            if parser.is_null() {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    6214 as ::core::ffi::c_int,
                    b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            let mut storage: CharData = CharData {
                count: 0,
                data: [0; 2048],
            };
            CharData_Init(&raw mut storage);
            XML_SetUserData(parser, &raw mut storage as *mut ::core::ffi::c_void);
            XML_SetStartElementHandler(
                parser,
                Some(
                    start_element_event_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *mut *const XML_Char,
                        ) -> (),
                ),
            );
            g_bytesScanned = 0 as ::core::ffi::c_uint;
            let mut worstcase_bytes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while *fillsize >= 0 as ::core::ffi::c_int {
                if !(offset + *fillsize <= document_length) {
                    _fail(
                        b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        6225 as ::core::ffi::c_int,
                        b"check failed: offset + *fillsize <= document_length\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                let status: XML_Status = XML_Parse(
                    parser,
                    document.offset(offset as isize) as *mut ::core::ffi::c_char,
                    *fillsize,
                    XML_FALSE as ::core::ffi::c_int,
                ) as XML_Status;
                if status as ::core::ffi::c_uint
                    != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    _xml_failure(
                        parser,
                        b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        6229 as ::core::ffi::c_int,
                    );
                }
                offset += *fillsize;
                fillsize = fillsize.offset(1);
                if !(offset <= 2147483647 as ::core::ffi::c_int - worstcase_bytes) {
                    _fail(
                        b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        6233 as ::core::ffi::c_int,
                        b"check failed: offset <= INT_MAX - worstcase_bytes\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                worstcase_bytes += offset;
            }
            if !(storage.count == 1 as ::core::ffi::c_int) {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    6236 as ::core::ffi::c_int,
                    b"check failed: storage.count == 1\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if !(g_bytesScanned > 0 as ::core::ffi::c_uint) {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    6237 as ::core::ffi::c_int,
                    b"check failed: g_bytesScanned > 0\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if g_reparseDeferralEnabledDefault != 0 {
                let max_bytes_scanned: ::core::ffi::c_uint = -*fillsize as ::core::ffi::c_uint;
                if g_bytesScanned > max_bytes_scanned {
                    fprintf(
                        stderr,
                        b"bytes scanned in parse attempts: actual=%u limit=%u \n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        g_bytesScanned,
                        max_bytes_scanned,
                    );
                    _fail(
                        b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        6245 as ::core::ffi::c_int,
                        b"too many bytes scanned in parse attempts\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            }
            if !(g_bytesScanned <= worstcase_bytes as ::core::ffi::c_uint) {
                _fail(
                    b"/root/work/expat/tests/basic_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    6248 as ::core::ffi::c_int,
                    b"check failed: g_bytesScanned <= (unsigned)worstcase_bytes\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            XML_ParserFree(parser);
            test_i += 1;
        }
        free(document as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn make_basic_test_case(mut s: *mut Suite) {
    unsafe {
        let mut tc_basic: *mut TCase =
            tcase_create(b"basic tests\0".as_ptr() as *const ::core::ffi::c_char);
        suite_add_tcase(s, tc_basic);
        tcase_add_checked_fixture(
            tc_basic,
            Some(basic_setup as unsafe extern "C" fn() -> ()),
            Some(basic_teardown as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_nul_byte as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_u0000_char as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_siphash_self as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_siphash_spec as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bom_utf8 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bom_utf16_be as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bom_utf16_le as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_nobom_utf16_le as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_hash_collision as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_illegal_utf8 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_utf8_auto_align as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(tc_basic, Some(test_utf16 as unsafe extern "C" fn() -> ()));
        tcase_add_test(
            tc_basic,
            Some(test_utf16_le_epilog_newline as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_not_utf16 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_encoding as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_latin1_umlauts as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_long_utf8_character as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_long_latin1_attribute as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_long_ascii_attribute as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_danish_latin1 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_french_charref_hexidecimal as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_french_charref_decimal as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_french_latin1 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_french_utf8 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_utf8_false_rejection as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_line_number_after_parse as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_column_number_after_parse as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_line_and_column_numbers_inside_handlers as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_line_number_after_error as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_column_number_after_error as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_really_long_lines as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_really_long_encoded_lines as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_end_element_events as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_helper_is_whitespace_normalized as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_attr_whitespace_normalization as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_xmldecl_misplaced as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_xmldecl_invalid as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_xmldecl_missing_attr as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_xmldecl_missing_value as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_unknown_encoding_internal_entity as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_unrecognised_encoding_internal_entity as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_ext_entity_set_encoding as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_ext_entity_no_handler as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_ext_entity_set_bom as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_ext_entity_bad_encoding as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_ext_entity_bad_encoding_2 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_wfc_undeclared_entity_unread_external_subset as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_wfc_undeclared_entity_no_external_subset as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_wfc_undeclared_entity_standalone as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(
                test_wfc_undeclared_entity_with_external_subset_standalone
                    as unsafe extern "C" fn() -> (),
            ),
        );
        tcase_add_test(
            tc_basic,
            Some(
                test_entity_with_external_subset_unless_standalone as unsafe extern "C" fn() -> (),
            ),
        );
        tcase_add_test(
            tc_basic,
            Some(test_wfc_undeclared_entity_with_external_subset as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_not_standalone_handler_reject as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_not_standalone_handler_accept as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_entity_start_tag_level_greater_than_one as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_wfc_no_recursive_entity_refs as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_no_indirectly_recursive_entity_refs as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_ext_entity_invalid_parse as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_dtd_default_handling as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_dtd_attr_handling as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_empty_ns_without_namespaces as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_ns_in_attribute_default_without_namespaces as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_stop_parser_between_char_data_calls as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_suspend_parser_between_char_data_calls as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_repeated_stop_parser_between_char_data_calls as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_good_cdata_ascii as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_good_cdata_utf16 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_good_cdata_utf16_le as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_long_cdata_utf16 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_multichar_cdata_utf16 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_utf16_bad_surrogate_pair as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_cdata as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_cdata_utf16 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_stop_parser_between_cdata_calls as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_suspend_parser_between_cdata_calls as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_memory_allocation as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_default_current as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_dtd_elements as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_dtd_elements_nesting as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_set_foreign_dtd as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_foreign_dtd_not_standalone as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_invalid_foreign_dtd as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_foreign_dtd_with_doctype as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_foreign_dtd_without_external_subset as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_empty_foreign_dtd as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_set_base as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_attributes as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_reset_in_entity as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_resume_invalid_parse as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_resume_resuspended as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_cdata_default as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_subordinate_reset as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_subordinate_suspend as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_subordinate_xdecl_suspend as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_subordinate_xdecl_abort as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_ext_entity_invalid_suspended_parse as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_explicit_encoding as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_trailing_cr as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_ext_entity_trailing_cr as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_trailing_rsqb as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_ext_entity_trailing_rsqb as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_ext_entity_good_cdata as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_user_parameters as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_ext_entity_ref_parameter as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_empty_parse as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_negative_len_parse as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_negative_len_parse_buffer as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_get_buffer_1 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_get_buffer_2 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_get_buffer_3_overflow as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_buffer_can_grow_to_max as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_getbuffer_allocates_on_zero_len as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_byte_info_at_end as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_byte_info_at_error as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_byte_info_at_cdata as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_predefined_entities as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_invalid_tag_in_dtd as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_not_predefined_entities as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_ignore_section as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_ignore_section_utf16 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_ignore_section_utf16_be as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_bad_ignore_section as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_external_bom_consumed as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_external_entity_values as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_ext_entity_not_standalone as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_ext_entity_value_abort as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_public_doctype as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_attribute_enum_value as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_predefined_entity_redefinition as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_dtd_stop_processing as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_public_notation_no_sysid as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_nested_groups as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_group_choice as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_standalone_parameter_entity as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_skipped_parameter_entity as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_recursive_external_parameter_entity as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_recursive_external_parameter_entity_2 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_undefined_ext_entity_in_external_dtd as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_suspend_xdecl as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_abort_epilog as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_abort_epilog_2 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_suspend_epilog as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_suspend_in_sole_empty_tag as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_unfinished_epilog as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_partial_char_in_epilog as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_suspend_resume_internal_entity as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_suspend_resume_internal_entity_issue_629 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_resume_entity_with_syntax_error as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_suspend_resume_parameter_entity as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_restart_on_error as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_reject_lt_in_attribute_value as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_reject_unfinished_param_in_att_value as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_trailing_cr_in_att_value as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_standalone_internal_entity as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_skipped_external_entity as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_skipped_null_loaded_ext_entity as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_skipped_unloaded_ext_entity as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_param_entity_with_trailing_cr as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_invalid_character_entity as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_invalid_character_entity_2 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_invalid_character_entity_3 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_invalid_character_entity_4 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_pi_handled_in_default as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_comment_handled_in_default as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(tc_basic, Some(test_pi_yml as unsafe extern "C" fn() -> ()));
        tcase_add_test(tc_basic, Some(test_pi_xnl as unsafe extern "C" fn() -> ()));
        tcase_add_test(tc_basic, Some(test_pi_xmm as unsafe extern "C" fn() -> ()));
        tcase_add_test(
            tc_basic,
            Some(test_utf16_pi as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_utf16_be_pi as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_utf16_be_comment as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_utf16_le_comment as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_missing_encoding_conversion_fn as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_failing_encoding_conversion_fn as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_unknown_encoding_success as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_unknown_encoding_bad_name as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_unknown_encoding_bad_name_2 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_unknown_encoding_long_name_1 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_unknown_encoding_long_name_2 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_invalid_unknown_encoding as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_unknown_ascii_encoding_ok as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_unknown_ascii_encoding_fail as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_unknown_encoding_invalid_length as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_unknown_encoding_invalid_topbit as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_unknown_encoding_invalid_surrogate as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_unknown_encoding_invalid_high as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_unknown_encoding_invalid_attr_value as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_unknown_encoding_user_data_primary as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_unknown_encoding_user_data_secondary as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_ext_entity_latin1_utf16le_bom as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_ext_entity_latin1_utf16be_bom as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_ext_entity_latin1_utf16le_bom2 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_ext_entity_latin1_utf16be_bom2 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_ext_entity_utf16_be as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_ext_entity_utf16_le as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_ext_entity_utf16_unknown as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_ext_entity_utf8_non_bom as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_utf8_in_cdata_section as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_utf8_in_cdata_section_2 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_utf8_in_start_tags as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_trailing_spaces_in_elements as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_utf16_attribute as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_utf16_second_attr as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_attr_after_solidus as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_utf16_pe as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_attr_desc_keyword as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_attr_desc_keyword_utf16 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_doctype as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_doctype_utf8 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_doctype_utf16 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_doctype_plus as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_doctype_star as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_doctype_query as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_unknown_encoding_bad_ignore as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_entity_in_utf16_be_attr as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_entity_in_utf16_le_attr as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_entity_public_utf16_be as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_entity_public_utf16_le as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_short_doctype as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_short_doctype_2 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_short_doctype_3 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_long_doctype as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_entity as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_entity_2 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_entity_3 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_entity_4 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bad_notation as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_default_doctype_handler as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_empty_element_abort as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_basic,
            Some(test_pool_integrity_with_unfinished_attr as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_entity_ref_no_elements as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_deep_nested_entity as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_deep_nested_attribute_entity as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_deep_nested_entity_delayed_interpretation as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_nested_entity_suspend as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_basic,
            Some(test_nested_entity_suspend_2 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_big_tokens_scale_linearly as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_set_reparse_deferral as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_reparse_deferral_is_inherited as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_set_reparse_deferral_on_null_parser as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_set_reparse_deferral_on_the_fly as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_set_bad_reparse_option as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_bypass_heuristic_when_close_to_bufsize as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_basic,
            Some(test_varying_buffer_fills as unsafe extern "C" fn() -> ()),
        );
    }
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
