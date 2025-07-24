FROM python:3.11

WORKDIR /etl

COPY requirements.txt .
RUN pip install -r requirements.txt

COPY ./etl ./etl
COPY run_etl.py .
COPY .env.example .env

CMD ["python", "run_etl.py"]
