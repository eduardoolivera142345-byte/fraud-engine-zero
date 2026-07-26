#!/bin/bash

echo "⚡ INICIANDO SIMULADOR BANCÁRIO REALISTA..."
echo "Disparando transações balanceadas para o Engine Rust na porta 50051"
echo "Pressione [CTRL+C] para encerrar."
echo "--------------------------------------------------------"

COUNT=1

while true; do
  # Sorteia um número de 1 a 10 para definir a probabilidade
  CHANCE=$((RANDOM % 10 + 1))

  if [ $CHANCE -le 8 ]; then
    # 80% das vezes: Transação normal/baixa (R$ 15,00 a R$ 850,00) -> APROVADA
    AMOUNT=$(( (RANDOM % 830) + 15 ))
  else
    # 20% das vezes: Transação alta/suspeita (R$ 12.000,00 a R$ 150.000,00) -> REJEITADA
    AMOUNT=$(( (RANDOM % 1380) * 100 + 12000 ))
  fi
  
  TX_ID="tx_sim_$(date +%s)_$COUNT"

  # Envia a transação via gRPC em background
  grpcurl -plaintext -d "{\"transaction_id\": \"$TX_ID\", \"amount\": $AMOUNT.0}" 127.0.0.1:50051 fraud.engine.v1.FraudEngineService/EvaluateTransaction > /dev/null 2>&1 &

  echo "[$COUNT] 💳 Transação enviada: $TX_ID | Valor: R$ $AMOUNT,00"

  # Intervalo de envio
  SLEEP_TIME="0.$(($RANDOM % 3 + 1))"
  sleep $SLEEP_TIME

  COUNT=$((COUNT + 1))
done
