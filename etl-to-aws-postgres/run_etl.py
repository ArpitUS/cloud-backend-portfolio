from etl.extract import extract
from etl.transform import transform
from etl.load import load

df = extract("etl/sample.csv")
df = transform(df)
load(df, "users")
