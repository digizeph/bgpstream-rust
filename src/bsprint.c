//
// Created by Mingwei Zhang on 2019-03-03.
//

#include "bgpstream.h"

#include <assert.h>
#include <ctype.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <unistd.h>

void print_bs_record(bgpstream_record_t *bs_record)
{
    // assert(bs_record);
    static char record_buf[65536];

    size_t written = 0; /* < how many bytes we wanted to write */
    ssize_t c = 0;      /* < how many chars were written */
    char *buf_p = record_buf;
    int len = 65536;

    /* record type */
    if ((c = bgpstream_record_dump_type_snprintf(
            buf_p, len - written, bs_record->attributes.dump_type)) < 0) {
        return;
    }
    written += c;
    buf_p += c;

    c = snprintf(buf_p, len - written, "|");
    written += c;
    buf_p += c;

    /* record position */
    if ((c = bgpstream_record_dump_pos_snprintf(buf_p, len - written,
                                                bs_record->dump_pos)) < 0) {
        return;
    }
    written += c;
    buf_p += c;

    /* Record timestamp, project, collector */
    c = snprintf(
            buf_p, len - written, "|%ld|%s|%s|", bs_record->attributes.record_time,
            bs_record->attributes.dump_project, bs_record->attributes.dump_collector);
    written += c;
    buf_p += c;

    /* record status */
    if ((c = bgpstream_record_status_snprintf(buf_p, len - written,
                                              bs_record->status)) < 0) {
        return;
    }
    written += c;
    buf_p += c;

    /* dump time */
    c = snprintf(buf_p, len - written, "|%ld", bs_record->attributes.dump_time);
    written += c;
    buf_p += c;

    if (written >= len) {
        return;
    }

    printf("%s\n", record_buf);
}
