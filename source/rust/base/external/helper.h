#include <raylib.h>

float DrawTextBoxed(Font font, const char *text, Rectangle rec, float fontSize, float spacing, bool wordWrap, Color tint);
float MeasureTextBoxed(Font font, const char *text, Rectangle rec, float fontSize, float spacing, bool wordWrap);

// raymath
Matrix MatrixTranslate(float x, float y, float z);
Matrix MatrixRotate(Vector3 axis, float angle);
Matrix MatrixScale(float x, float y, float z);
Matrix MatrixIdentity();
Matrix MatrixMultiply(Matrix left, Matrix right);