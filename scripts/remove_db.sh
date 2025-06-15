
# Create USER and DATABASE Dominote to managew with your own user
sudo -u postgres psql -c "DROP USER ${LOGNAME};"
sudo -u postgres psql -c "DROP DATABASE dominote"
