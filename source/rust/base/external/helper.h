#include <raylib.h>

float DrawTextBoxed(Font font, const char *text, Rectangle rec, float fontSize, float spacing, bool wordWrap, Color tint);
float MeasureTextBoxed(Font font, const char *text, Rectangle rec, float fontSize, float spacing, bool wordWrap);