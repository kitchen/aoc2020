for i in {55..906} ; do echo "${i} $(grep -w -L ${i} seats)" ; done | grep seats
