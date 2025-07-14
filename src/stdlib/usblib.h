//positional
void print(char* input);
char* input(char* prompt);
char* get(char* prompt);
long day(long days);
long hour(long hours);
long minutes(long minutes);
long seconds(long seconds);
long date();
void cscope();
long poptions(char options[], char* prompt);
long menu(long choice);
void penum(long enum_value);
void time(long date_value);
//enclosed
char* str(unsigned int type, void* value);
long _int(unsigned int type, void* value);
float _float(unsigned int type, void* value);
bool _bool(unsigned int type, void* value);
long sgn(long value);
long abs(long value);
long sqr(long value);
long rnd(long value);
char* left(char* original);
char* right(char* original);
char* mid(char* original);

